use super::*;

use crate::board::Board;
use crate::board::Move;
use crate::board::Pos;
use crate::board::pid;

pub struct NaiveUIter {
	nxt:fn(&mut Self, &Board)->Option<Move>,
	data:IterData,
	from:Pos,
}
union IterData {
	none:(),
	nc:IterDataNC,
	mc:IterDataMC,
	pc:IterDataPC,
}
#[derive(Clone, Copy)] struct IterDataNC {
	nxt:fn(&mut NaiveUIter, &Board)->Option<Move>,
	from:Pos,
	to:Pos,
	index:u8,
	to_range:Pos,
	grid:u8,
	pin:u8,
	pawn:[bool; 9],
}
#[derive(Clone, Copy)] struct IterDataMC {
	index:u8,
	kpos:Pos,
	num:u8,
	apos:[Pos; 10],
}
#[derive(Clone, Copy)] struct IterDataPC {
	kflee:&'static[Pos],
	index:u8,
}
impl Default for NaiveUIter {
	fn default()->Self {
		return Self{
			nxt:Self::next_dead,
			data:IterData{none:()},
			from:unsafe {std::mem::zeroed()},
		};
	}
}
impl Board {
	pub fn moves_unbound(&self)->NaiveUIter {
		let cpos = self.checker();
		match cpos {
			Pos::REMOVED => {
				fn two_pawn(b:&Board)->[bool; 9] {
					let mut pawns = [false; 9];
					let pawn = if b.turn {
						{pid::PAWN | 0x10}
					} else {pid::PAWN} as u8 + 1;
					for x in 0 .. 9 {
						for y in 0 .. 9 {
							if b.terra[y][x] == pawn {
								pawns[x] = true;
								break;
							}
						}
					}
					return pawns;
				}
				
				let mut iter = NaiveUIter{
					nxt:NaiveUIter::next_dead,
					data:IterData{nc:IterDataNC{
						nxt:NaiveUIter::next_dead,
						from:unsafe {std::mem::zeroed()},
						to:unsafe {std::mem::zeroed()},
						index:unsafe {std::mem::zeroed()},
						to_range:unsafe {std::mem::zeroed()},
						grid:unsafe {std::mem::zeroed()},
						pin:unsafe {std::mem::zeroed()},
						pawn:two_pawn(self),
					}},
					from:Pos{x:0, y:0},
				};
				iter.choose_next_nocheck::<true>(self);
				return iter;
			}
			Pos::RESERVE =>
				return NaiveUIter{
					nxt:NaiveUIter::next_polycheck,
					data:IterData{pc:IterDataPC{
						index:0,
						kflee:LIST_MOVE_MELEE[if self.turn {
							{pid::KING | 0x10}
						} else {pid::KING}],
					}},
					from:self.kpos[self.turn as usize],
				},
			_ => {
				let kpos = self.kpos[self.turn as usize];
				let mut iter = NaiveUIter{
					nxt:NaiveUIter::next_dead,
					data:IterData{mc:IterDataMC{
						index:unsafe {std::mem::zeroed()},
						kpos:self.kpos[self.turn as usize],
						num:unsafe {std::mem::zeroed()},
						apos:unsafe {std::mem::zeroed()},
					}},
					from:cpos,
				};
				let diff = kpos - cpos;
				match diff {					
					Pos{x:0, y:1..} => iter.choose_next_monocheck::<0, 1, true>(self),
					Pos{x:0, y:..=-1} => iter.choose_next_monocheck::<0, -1, true>(self),
					Pos{x:1.., y:0} => iter.choose_next_monocheck::<1, 0, true>(self),
					Pos{x:..=-1, y:0} => iter.choose_next_monocheck::<-1, 0, true>(self),
					_ => match diff.turn45() {
						Pos{x:0, y:1..} => iter.choose_next_monocheck::<1, 1, true>(self),
						Pos{x:0, y:..=-1} => iter.choose_next_monocheck::<-1, -1, true>(self),
						Pos{x:1.., y:0} => iter.choose_next_monocheck::<1, -1, true>(self),
						Pos{x:..=-1, y:0} => iter.choose_next_monocheck::<-1, 1, true>(self),
						_ => iter.choose_next_monocheck::<1, 2, true>(self),
					},
				}
				return iter;
			}
		}
	}
}
impl NaiveUIter {
	pub fn next(&mut self, b:&Board)->Option<Move> {(self.nxt)(self, b)}
	
	fn next_dead(&mut self, _:&Board)->Option<Move> {None}
	
	//No check
	fn choose_next_nocheck<const INIT:bool>(&mut self, b:&Board) {
		let nc = unsafe {&mut self.data.nc};
		
		let p = &mut self.from;
		if !INIT {
			p.x += 1;
			if p.x >= 9 {
				p.y += 1;
				if p.y >= 9 {
					self.nxt = Self::next_dead;
					return;
				}
				p.x = 0;
			}
		}
		loop {
			let mut grid = b.terra[p.y as usize][p.x as usize];
			if grid > 0 {
				grid -= 1;
				if (grid & 0x10 != 0) == b.turn {
					if grid as usize & 7 == pid::KING {
						self.nxt = Self::next_noncheck_terra_melee::<true>;
					}
					else {
						self.nxt = Self::next_noncheck_terra_melee::<false>;
						nc.pin = b.pinned(*p, !b.turn, b.kpos[b.turn as usize]);
					};
					nc.index = 0;
					nc.grid = grid & 0x1F;
					break;
				}
			}
			else {
				self.nxt = Self::next_noncheck_drop;
				nc.index = 0;
				break;
			}
			
			p.x += 1;
			if p.x >= 9 {
				p.y += 1;
				if p.y >= 9 {
					self.nxt = Self::next_dead;
					break;
				}
				p.x = 0;
			}
		}
	}
	
	fn next_noncheck_anp(&mut self, _:&Board)->Option<Move> {
		let nc = unsafe {&mut self.data.nc};
		
		self.nxt = nc.nxt;
		return Some(Move{
			from:nc.from,
			to:nc.to,
			prmt:false,
		});
	}
	
	fn next_noncheck_drop(&mut self, b:&Board)->Option<Move> {
		let nc = unsafe {&mut self.data.nc};
		
		while nc.index < 7 {
			let index = nc.index as usize;
			nc.index += 1;
			
			if b.rsv[b.turn as usize][index] == 0 {continue}
			
			let agent = if b.turn {index | 0x10} else {index};
			if CAN_PRMT[agent][self.from.y as usize] == 3 {continue}
			if index == pid::PAWN {
				if nc.pawn[self.from.x as usize] {continue}
				if b.pawn_drop_mate(agent as u8, self.from) {continue}
			}
			
			return Some(Move{
				from:Pos{
					x:if b.turn {10} else {9},
					y:index as i8,
				},
				to:self.from,
				prmt:false,
			});
		}
		
		self.choose_next_nocheck::<false>(b);
		return (self.nxt)(self, b);
	}
	
	fn next_noncheck_terra_melee<const KING:bool>(&mut self, b:&Board)->Option<Move> {
		let nc = unsafe {&mut self.data.nc};
		
		let list = LIST_MOVE_MELEE[nc.grid as usize];
		
		'a:while (nc.index as usize) < list.len() {
			let diff = list[nc.index as usize];
			nc.index += 1;
			
			if !KING {
				match nc.pin {
					pin::HRZ => if diff.y != 0 {continue},
					pin::VRT => if diff.x != 0 {continue},
					pin::POS => if diff.x != diff.y {continue},
					pin::NEG => if diff.x + diff.y != 0 {continue},
					_ => {}
				}
			}
			
			nc.to = self.from + diff;
			let Pos{x:0 .. 9, y:0 .. 9} = nc.to else {continue};
			let victim = b.terra[nc.to.y as usize][nc.to.x as usize];
			if victim > 0 && (victim - 1 & 0x10 != 0) == b.turn {
				continue;
			}
			
			if KING {
				let to = nc.to;
				find_attacker!(!b.turn, b, to, {continue 'a});
			}
			
			match CAN_PRMT[nc.grid as usize][self.from.y as usize]
				| CAN_PRMT[nc.grid as usize][nc.to.y as usize] {
				1 => {
					nc.nxt = self.nxt;
					self.nxt = Self::next_noncheck_anp;
					nc.from = self.from;
					return Some(Move{
						from:nc.from,
						to:nc.to,
						prmt:true,
					});
				},
				3 => return Some(Move{
					from:self.from,
					to:nc.to,
					prmt:true,
				}),
				_ => return Some(Move{
					from:self.from,
					to:nc.to,
					prmt:false,
				}),
			}
		}
		
		nc.index = 0;
		self.next_noncheck_terra_ranged_nd(b);
		return (self.nxt)(self, b);
	}
	
	fn next_noncheck_terra_ranged<const DX:i8, const DY:i8, const XFIRST:bool>(&mut self, b:&Board)->Option<Move> {
		let nc = unsafe {&mut self.data.nc};
		
		if nc.to_range.find_grid_border::<DX, DY, XFIRST>() {
			nc.from = self.from;
			nc.to = nc.to_range;
			let grid = nc.grid as usize;
			
			let victim = b.terra[nc.to.y as usize][nc.to.x as usize];
			if victim > 0 {
				self.next_noncheck_terra_ranged_nd(b);
				if (victim - 1 & 0x10 != 0) == b.turn {
					return (self.nxt)(self, b);
				}
			}
			else {
				nc.to_range.find_grid_inc::<DX, DY>();
			}	
			
			let nc = unsafe {&mut self.data.nc};
			
			match CAN_PRMT[grid][nc.from.y as usize]
				| CAN_PRMT[grid][nc.to.y as usize] {
				1 => {
					nc.nxt = self.nxt;
					self.nxt = Self::next_noncheck_anp;
					return Some(Move{
						from:nc.from,
						to:nc.to,
						prmt:true,
					});
				}
				3 => return Some(Move{
					from:nc.from,
					to:nc.to,
					prmt:true,
				}),
				_ => return Some(Move{
					from:nc.from,
					to:nc.to,
					prmt:false,
				}),
			}
		}
		
		self.next_noncheck_terra_ranged_nd(b);
		return (self.nxt)(self, b);
	}
	
	fn next_noncheck_terra_ranged_nd(&mut self, b:&Board) {
		let nc = unsafe {&mut self.data.nc};
		
		let list = LIST_MOVE_RANGED[nc.grid as usize];
		
		while (nc.index as usize) < list.len() {
			let dir = list[nc.index as usize];
			nc.index += 1;
			
			macro_rules! r {
				($dx:literal, $dy:literal, $pin:pat) => {
					if let pin::NO | $pin = nc.pin {
						self.nxt = if self.from.find_grid_xfirst::<$dx, $dy>() {
							{Self::next_noncheck_terra_ranged::<$dx, $dy, true>}
						}
						else {Self::next_noncheck_terra_ranged::<$dx, $dy, false>};
						nc.to_range = self.from + dir;
						return;
					}
				};
			}
			
			match dir {
				Pos{x:0, y:1} => r!(0, 1, pin::VRT),
				Pos{x:0, y:-1} => r!(0, -1, pin::VRT),
				Pos{x:1, y:0} => r!(1, 0, pin::HRZ),
				Pos{x:-1, y:0} => r!(-1, 0, pin::HRZ),
				Pos{x:1, y:1} => r!(1, 1, pin::POS),
				Pos{x:-1, y:-1} => r!(-1, -1, pin::POS),
				Pos{x:1, y:-1} => r!(1, -1, pin::NEG),
				Pos{x:-1, y:1} => r!(-1, 1, pin::NEG),
				_ => {}
			}
		}
		
		self.choose_next_nocheck::<false>(b);
	}
	
	//Single check
	fn choose_next_monocheck<const DX:i8, const DY:i8, const INIT:bool>(&mut self, b:&Board) {
		let mc = unsafe {&mut self.data.mc};
		
		if !INIT {
			if match (DX, DY) {
				(1 | -1, 1 | -1) | (1 | -1, 0) | (0, 1 | -1) => {
					self.from.find_grid_inc::<DX, DY>();
					{!self.from.find_grid_end::<DX, DY>(mc.kpos)}
				}
				_ => {
					self.from = mc.kpos;
					{true}
				},
			} {
				*unsafe {&mut self.data.pc} = IterDataPC{
					index:0,
					kflee:LIST_MOVE_MELEE[if b.turn {
						{pid::KING | 0x10}
					} else {pid::KING}],
				};
				self.nxt = Self::next_polycheck;
				return;
			}
		}
		
		self.nxt = Self::next_monocheck_terra::<DX, DY>;

		mc.index = 0;
		mc.num = 0;
		let pos = self.from;
		let mut apos;
		let mut grid;
		find_attacker!(b.turn, b, pos, apos, grid, {
			if apos != mc.kpos && b.pinned(apos, !b.turn, mc.kpos) == pin::NO {				
				mc.apos[mc.num as usize] = apos;
				mc.num += 1;
			}
		});
	}
	
	fn next_monocheck_anp<const DX:i8, const DY:i8>(&mut self, _:&Board)->Option<Move> {
		let mc = unsafe {&mut self.data.mc};
		
		self.nxt = Self::next_monocheck_terra::<DX, DY>;
		return Some(Move{
			from:mc.apos[mc.index as usize - 1],
			to:self.from,
			prmt:false,
		});
	}
	
	fn next_monocheck_terra<const DX:i8, const DY:i8>(&mut self, b:&Board)->Option<Move> {
		let mc = unsafe {&mut self.data.mc};
		
		if mc.index < mc.num {
			let from = mc.apos[mc.index as usize];
			let to = self.from;
			mc.index += 1;
			
			let agent = b.terra[from.y as usize][from.x as usize] - 1;
			
			match CAN_PRMT[agent as usize][to.y as usize]
				| CAN_PRMT[agent as usize][from.y as usize] {
				1 => {
					self.nxt = Self::next_monocheck_anp::<DX, DY>;
					return Some(Move{
						from,
						to,
						prmt:true,
					});
				}
				3 => return Some(Move{
					from,
					to,
					prmt:true,
				}),
				_ => return Some(Move{
					from,
					to,
					prmt:false,
				}),
			}
		}
		
		if b.terra[self.from.y as usize][self.from.x as usize] == 0 {
			self.nxt = Self::next_monocheck_drop::<DX, DY>;
			mc.index = 0;
			return self.next_monocheck_drop::<DX, DY>(b);
		}
		else {
			self.choose_next_monocheck::<DX, DY, false>(b);
			return (self.nxt)(self, b);
		}
	}
	
	fn next_monocheck_drop<const DX:i8, const DY:i8>(&mut self, b:&Board)->Option<Move> {
		let mc = unsafe {&mut self.data.mc};
		
		'a:while mc.index < 7 {
			let index = mc.index as usize;
			mc.index += 1;
			
			if b.rsv[b.turn as usize][index] == 0 {continue}
			
			if CAN_PRMT[if b.turn {index | 0x10} else {index}][self.from.y as usize] == 3 {
				continue;
			}
			
			if index == pid::PAWN {
				let pawn = if b.turn {pid::PAWN | 0x10} else {pid::PAWN} as u8 + 1;
				for y in 0 .. 9 {
					if b.terra[y][self.from.x as usize] == pawn {continue 'a}
				}
				if b.pawn_drop_mate(pawn, self.from) {continue}
			}
			
			return Some(Move{
				from:Pos{
					x:if b.turn {10} else {9},
					y:index as i8,
				},
				to:self.from,
				prmt:false,
			});
		}
		
		self.choose_next_monocheck::<DX, DY, false>(b);
		return (self.nxt)(self, b);
	}
	
	//Double check
	fn next_polycheck(&mut self, b:&Board)->Option<Move> {
		let pc = unsafe {&mut self.data.pc};
		
		'a:while (pc.index as usize) < pc.kflee.len() {
			let to = self.from + pc.kflee[pc.index as usize];
			pc.index += 1;
			
			let Pos{x:0 .. 9, y:0 .. 9} = to else {continue};
			let victim = b.terra[to.y as usize][to.x as usize];
			if victim > 0 && (victim - 1 & 0x10 != 0) == b.turn {
				continue;
			}
			
			find_attacker!(!b.turn, b, to, {continue 'a});
			if b.pinned(self.from, !b.turn, to) != pin::NO {continue}
			
			return Some(Move{
				from:self.from,
				to,
				prmt:false,
			});
		}
		
		self.nxt = Self::next_dead;
		return None;
	}
}

pub struct NaiveIter<'a> {
	uiter:NaiveUIter,
	b:&'a Board,
}
impl Board {
	pub fn moves(&self)->NaiveIter {
		return NaiveIter{uiter:self.moves_unbound(), b:self};
	}
}
impl Iterator for NaiveIter<'_> {
	type Item = Move;
	fn next(&mut self)->Option<Move> {self.uiter.next(self.b)}
}