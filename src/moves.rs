use crate::board::Board;
use crate::board::Move;
use crate::board::Pos;
use crate::board::pid;
use crate::board::player;

pub const CAN_MOVE:[[[u8; 17]; 17]; 0x20] = init::can_move();

pub const LIST_MOVE_MELEE:[&[Pos]; 0x20] = init::list_move_melee();
pub const LIST_MOVE_RANGED:[&[Pos]; 0x20] = init::list_move_ranged();

pub const CAN_PRMT:[[u8; 9]; 0x20] = init::can_prmt();

pub const ATTACKERS_MELEE:[[[[bool; 0x20]; 3]; 3]; 2] = init::attackers_melee();
pub const ATTACKERS_RANGED:[[[[bool; 0x20]; 3]; 3]; 2] = init::attackers_ranged();

impl Pos {
	pub fn find_grid_inc<const DX:i8, const DY:i8>(&mut self) {
		match (DX, DY) {
			(0, 1) => self.y += 1,
			(0, -1) => self.y -= 1,
			(1, 0) => self.x += 1,
			(-1, 0) => self.x -= 1,
			(1, 1) => { self.x += 1; self.y += 1; }
			(-1, -1) => { self.x -= 1; self.y -= 1; }
			(1, -1) => { self.x += 1; self.y -= 1; }
			(-1, 1) => { self.x -= 1; self.y += 1; }
			_ => unreachable!{}
		}
	}

	pub fn find_grid_xfirst<const DX:i8, const DY:i8>(self)->bool {
		match (DX, DY) {
			(0, 1) => false,
			(0, -1) => false,
			(1, 0) => true,
			(-1, 0) => true,
			(1, 1) => self.x >= self.y,
			(-1, -1) => self.x <= self.y,
			(1, -1) => self.x + self.y >= 8,
			(-1, 1) => self.x + self.y <= 8,
			_ => unreachable!{}
		}
	}

	pub fn find_grid_border<const DX:i8, const DY:i8, const XFIRST:bool>(self)->bool {
		return match (DX, DY, XFIRST) {
			(0, 1, _) | (1 | -1, 1, false) => self.y < 9,
			(0, -1, _) | (1 | -1, -1, false) => self.y >= 0,
			(1, 0, _) | (1, 1 | -1, true) => self.x < 9,
			(-1, 0, _) | (-1, 1 | -1, true) => self.x >= 0,
			_ => unreachable!{}
		};
	}

	pub fn find_grid_end<const DX:i8, const DY:i8>(self, end:Pos)->bool {
		return match (DX, DY) {
			(0, 1) => self.y < end.y,
			(0, -1) => self.y > end.y,
			(1, 0 | 1 | -1) => self.x < end.x,
			(-1, 0 | 1 | -1) => self.x > end.x,
			_ => unreachable!{}
		};
	}
}

#[macro_export] macro_rules! search_seg {
	($dx:literal, $dy:literal, $b:ident, $pos:ident, $end:ident, $grid:ident, $action:block) => {
		$pos.find_grid_inc::<$dx, $dy>();
		while $pos.find_grid_end::<$dx, $dy>($end) {
			$grid = $b.terra[$pos.y as usize][$pos.x as usize];
			if $grid > 0 $action
			$pos.find_grid_inc::<$dx, $dy>();
		}
	};
	($b:ident, $pos:ident, $end:ident, $grid:ident, $action:block) => {
		let diff = $end - $pos;
		match diff {
			Pos{x:0, y:1..} => {search_seg!(0, 1, $b, $pos, $end, $grid, $action);}
			Pos{x:0, y:..=-1} => {search_seg!(0, -1, $b, $pos, $end, $grid, $action);}
			Pos{x:1.., y:0} => {search_seg!(1, 0, $b, $pos, $end, $grid, $action);}
			Pos{x:..=-1, y:0} => {search_seg!(-1, 0, $b, $pos, $end, $grid, $action);}
			_ => match diff.turn45() {
				Pos{x:0, y:1..} => {search_seg!(1, 1, $b, $pos, $end, $grid, $action);}
				Pos{x:0, y:..=-1} => {search_seg!(-1, -1, $b, $pos, $end, $grid, $action);}
				Pos{x:1.., y:0} => {search_seg!(1, -1, $b, $pos, $end, $grid, $action);}
				Pos{x:..=-1, y:0} => {search_seg!(-1, 1, $b, $pos, $end, $grid, $action);}
				_ => {}
			},
		}
	};
}

#[macro_export] macro_rules! search_ray {
	($dx:literal, $dy:literal, $xfirst:literal, $b:ident, $pos:ident, $grid:ident, $action:block) => {
		$pos.find_grid_inc::<$dx, $dy>();
		while $pos.find_grid_border::<$dx, $dy, $xfirst>() {
			$grid = $b.terra[$pos.y as usize][$pos.x as usize];
			$action
			$pos.find_grid_inc::<$dx, $dy>();
		}
	};
	($diff:expr, $b:ident, $pos:ident, $grid:ident, $action:block) => {
		let diff = $diff;
		match diff {
			Pos{x:0, y:1..} => {
				if $pos.find_grid_xfirst::<0, 1>() {
					search_ray!(0, 1, true, $b, $pos, $grid, $action);
				}
				else {search_ray!(0, 1, false, $b, $pos, $grid, $action);}
			}
			Pos{x:0, y:..=-1} => {
				if $pos.find_grid_xfirst::<0, -1>() {
					search_ray!(0, -1, true, $b, $pos, $grid, $action);
				}
				else {search_ray!(0, -1, false, $b, $pos, $grid, $action);}
			}
			Pos{x:1.., y:0} => {
				if $pos.find_grid_xfirst::<1, 0>() {
					search_ray!(1, 0, true, $b, $pos, $grid, $action);
				}
				else {search_ray!(1, 0, false, $b, $pos, $grid, $action);}
			}
			Pos{x:..=-1, y:0} => {
				if $pos.find_grid_xfirst::<-1, 0>() {
					search_ray!(-1, 0, true, $b, $pos, $grid, $action);
				}
				else {search_ray!(-1, 0, false, $b, $pos, $grid, $action);}
			}
			_ => match diff.turn45() {
				Pos{x:0, y:1..} => {
					if $pos.find_grid_xfirst::<1, 1>() {
						search_ray!(1, 1, true, $b, $pos, $grid, $action);
					}
					else {search_ray!(1, 1, false, $b, $pos, $grid, $action);}
				}
				Pos{x:0, y:..=-1} => {
					if $pos.find_grid_xfirst::<-1, -1>() {
						search_ray!(-1, -1, true, $b, $pos, $grid, $action);
					}
					else {search_ray!(-1, -1, false, $b, $pos, $grid, $action);}
				}
				Pos{x:1.., y:0} => {
					if $pos.find_grid_xfirst::<1, -1>() {
						search_ray!(1, -1, true, $b, $pos, $grid, $action);
					}
					else {search_ray!(1, -1, false, $b, $pos, $grid, $action);}
				}
				Pos{x:..=-1, y:0} => {
					if $pos.find_grid_xfirst::<-1, 1>() {
						search_ray!(-1, 1, true, $b, $pos, $grid, $action);
					}
					else {search_ray!(-1, 1, false, $b, $pos, $grid, $action);}
				}
				_ => {}
			},
		}
	}
}

macro_rules! find_attacker_ranged {
	($dx:literal, $dy:literal, $xfirst:literal, $b:ident, $pos:ident, $grid:ident, $action:block) => {
		search_ray!($dx, $dy, $xfirst, $b, $pos, $grid, {
			if $grid > 0 {
				$grid -= 1;
				$action	
			}
		});
	};
}

macro_rules! find_attacker_noarcher {
	(b, $dx:literal, $dy:literal, $xfirst:literal, $b:ident, $pos:ident, $grid:ident, $action:block, $action_ranged:block) => {
		$pos.find_grid_inc::<$dx, $dy>();
		if $pos.find_grid_border::<$dx, $dy, $xfirst>() {
			$grid = $b.terra[$pos.y as usize][$pos.x as usize];
			if $grid > 0 {
				$grid -= 1;
				$action
			}
			else {
				find_attacker_ranged!{$dx, $dy, $xfirst, $b, $pos, $grid, $action_ranged}
			}
		}
	};
	($dx:literal, $dy:literal, $agrsor:literal, $b:ident, $pos:ident, $grid:ident, $action:block, $action_ranged:block) => {
		if $pos.find_grid_xfirst::<$dx, $dy>() {
			find_attacker_noarcher!(b, $dx, $dy, true, $b, $pos, $grid, $action, $action_ranged);
		} else {
			find_attacker_noarcher!(b, $dx, $dy, false, $b, $pos, $grid, $action, $action_ranged);
		}
	};
}

macro_rules! find_attacker_archer {
	($agrsor:literal, $b:ident, $pos:ident, $apos:ident, $grid:ident, $action:block) => {
		if $agrsor {
			if $pos.y >= 2 {
				if $pos.x < 8 {
					$apos = Pos{x:$pos.x + 1, y:$pos.y - 2};
					$grid = $b.terra[$apos.y as usize][$apos.x as usize];
					if $grid > 0 {
						$grid -= 1;
						if $grid as usize == pid::ARCHER | 0x10
							$action
					}
				}
				if $pos.x >= 1 {
					$apos = Pos{x:$pos.x - 1, y:$pos.y - 2};
					$grid = $b.terra[$apos.y as usize][$apos.x as usize];
					if $grid > 0 {
						$grid -= 1;
						if $grid as usize == pid::ARCHER | 0x10
							$action
					}
				}
			}
		}
		else {
			if $pos.y < 7 {
				if $pos.x < 8 {
					$apos = Pos{x:$pos.x + 1, y:$pos.y + 2};
					$grid = $b.terra[$apos.y as usize][$apos.x as usize];
					if $grid > 0 {
						$grid -= 1;
						if $grid as usize == pid::ARCHER
							$action
					}
				}
				if $pos.x >= 1 {
					$apos = Pos{x:$pos.x - 1, y:$pos.y + 2};
					$grid = $b.terra[$apos.y as usize][$apos.x as usize];
					if $grid > 0 {
						$grid -= 1;
						if $grid as usize == pid::ARCHER
							$action
					}
				}
			}
		}
	};
}

macro_rules! find_attacker {
	($agrsor:expr, $b:ident, $pos:ident, $apos:ident, $grid:ident, $action:block) => {
		if $agrsor {
			find_attacker!(d, 0, 1, true, $b, $pos, $apos, $grid, $action);
			find_attacker!(d, 0, -1, true, $b, $pos, $apos, $grid, $action);
			find_attacker!(d, 1, 0, true, $b, $pos, $apos, $grid, $action);
			find_attacker!(d, -1, 0, true, $b, $pos, $apos, $grid, $action);
			find_attacker!(d, 1, 1, true, $b, $pos, $apos, $grid, $action);
			find_attacker!(d, -1, -1, true, $b, $pos, $apos, $grid, $action);
			find_attacker!(d, 1, -1, true, $b, $pos, $apos, $grid, $action);
			find_attacker!(d, -1, 1, true, $b, $pos, $apos, $grid, $action);
			find_attacker_archer!(true, $b, $pos, $apos, $grid, $action);
		}
		else {
			find_attacker!(d, 0, 1, false, $b, $pos, $apos, $grid, $action);
			find_attacker!(d, 0, -1, false, $b, $pos, $apos, $grid, $action);
			find_attacker!(d, 1, 0, false, $b, $pos, $apos, $grid, $action);
			find_attacker!(d, -1, 0, false, $b, $pos, $apos, $grid, $action);
			find_attacker!(d, 1, 1, false, $b, $pos, $apos, $grid, $action);
			find_attacker!(d, -1, -1, false, $b, $pos, $apos, $grid, $action);
			find_attacker!(d, 1, -1, false, $b, $pos, $apos, $grid, $action);
			find_attacker!(d, -1, 1, false, $b, $pos, $apos, $grid, $action);
			find_attacker_archer!(false, $b, $pos, $apos, $grid, $action);
		}
		
	};
	($agrsor:expr, $b:ident, $pos:ident, $action:block) => {
		let mut apos;
		let mut grid;
		find_attacker!($agrsor, $b, $pos, apos, grid, $action);
	};
	(d, $dx:literal, $dy:literal, $agrsor:literal, $b:ident, $pos:ident, $apos:ident, $grid:ident, $action:block) => {
		$apos = $pos;
		find_attacker_noarcher!($dx, $dy, $agrsor, $b, $apos, $grid, {
			if ATTACKERS_MELEE[$agrsor as usize][($dx + 1) as usize][($dy + 1) as usize][$grid as usize & 0x1F]
				$action
		}, {
			if ATTACKERS_RANGED[$agrsor as usize][($dx + 1) as usize][($dy + 1) as usize][$grid as usize & 0x1F]
				$action
			else {break}
		});
	};
}

pub mod pin {
	pub const NO:u8 = 0;
	pub const VRT:u8 = 1;
	pub const HRZ:u8 = 2;
	pub const POS:u8 = 3;
	pub const NEG:u8 = 4;
}

impl Board {
	/*
	* Returns:
	*   0, good;
	*   1, unknown piece,
	*   2, black has plural king;
	*   3, white has plural king;
	*   4, black kpos incorrect;
	*   5, white kpos incorrect;
	*   6, checked out of turn;
	*/
	pub fn valid(&self)->u8 {
		let mut kpos_b = false;
		let mut kpos_w = false;
		for x in 0 .. 9 {
			for y in 0 .. 9 {
				let grid = self.terra[y as usize][x as usize] as usize;
				
				if grid > 0x20 {return 1}
				
				if grid == pid::KING + 1 {
					if kpos_b {return 2}
					else {kpos_b = true}
					
					if self.kpos[0] != (Pos{x, y}) {
						return 4;
					}
				}
				else if grid == (pid::KING | 0x10) + 1 {
					if kpos_w {return 3}
					else {kpos_w = true}
					
					if self.kpos[1] != (Pos{x, y}) {
						return 5;
					}
				}
			}
		}
		if !kpos_b && self.kpos[0] != Pos::REMOVED {
			return 4;
		}
		if !kpos_w && self.kpos[1] != Pos::REMOVED {
			return 5;
		}
		
		let kpos = self.kpos[!self.turn as usize];
		if kpos != Pos::REMOVED {
			find_attacker!(self.turn, self, kpos, {return 6});
		}
		
		return 0;
	}
	
	/*
	* Returns how a real/imaginary piece at [pos] would block Player [agrsor] from attacking [target]
	*/
	pub fn pinned(&self, pos:Pos, agrsor:bool, target:Pos)->u8 {
		let mut from = target;
		let diff = pos - from;
		let mut grid;
		macro_rules! p {
			($dx:literal, $dy:literal, $xfirst:literal, $agrsor:literal, $pin:expr) => {
				search_seg!($dx, $dy, self, from, pos, grid, {return pin::NO});
				find_attacker_ranged!($dx, $dy, $xfirst, self, from, grid, {
					return if ATTACKERS_RANGED[$agrsor as usize][($dx + 1) as usize][($dy + 1) as usize][grid as usize & 0x1F] {
						{$pin}
					} else {pin::NO};
				});
				return pin::NO;
			};
			($dx:literal, $dy:literal, $pin:expr) => {
				if pos.find_grid_xfirst::<$dx, $dy>() {
					if agrsor {
						p!($dx, $dy, true, true, $pin);
					}
					else {
						p!($dx, $dy, true, false, $pin);
					}
				}
				else {
					if agrsor {
						p!($dx, $dy, false, true, $pin);
					}
					else {
						p!($dx, $dy, false, false, $pin);
					}
				}
			};
		}
		match diff {
			Pos{x:0, y:1..} => p!(0, 1, pin::VRT),
			Pos{x:0, y:..=-1} => p!(0, -1, pin::VRT),
			Pos{x:1.., y:0} => p!(1, 0, pin::HRZ),
			Pos{x:..=-1, y:0} => p!(-1, 0, pin::HRZ),
			_ => match diff.turn45() {
				Pos{x:0, y:1..} => p!(1, 1, pin::POS),
				Pos{x:0, y:..=-1} => p!(-1, -1, pin::POS),
				Pos{x:1.., y:0} => p!(1, -1, pin::NEG),
				Pos{x:..=-1, y:0} => p!(-1, 1, pin::NEG),
				_ => return pin::NO,
			},
		}
	}

	pub fn legal(&self, mv:Move)->bool {
		let Move{from, to, prmt} = mv;
		let Pos{x:0 .. 9, y:0 .. 9} = to else {return false};
		if let 0 .. 9 = from.x {
			let 0 .. 9 = from.y else {return false};
			
			let mut agent = self.terra[from.y as usize][from.x as usize];
			if agent == 0 { return false; }
			agent -= 1;
			
			let owner = agent & 16 != 0;
			if owner != self.turn { return false; }
			
			let victim = self.terra[to.y as usize][to.x as usize];
			if victim > 0 && (victim - 1 & 16 != 0) == owner { return false; }
			
			let diff = to - from;
			match CAN_MOVE[(agent & 0x1F) as usize][(diff.x + 8) as usize][(diff.y + 8) as usize] {
				1 => {}
				2 => {
					let mut pos = from;
					let mut grid;
					search_seg!(self, pos, to, grid, {return false});
				}
				_ => return false,
			}
			
			if agent as usize & 7 == pid::KING {
				find_attacker!(!owner, self, to, {return false});
				if self.pinned(from, !owner, to) != pin::NO { return false; }
				if prmt { return false; }
			}
			else {
				let cpos = self.checker();
				match cpos {
					Pos::REMOVED => {
						match self.pinned(from, !owner, self.kpos[owner as usize]) {
							pin::VRT => if from.x != to.x { return false; },
							pin::HRZ => if from.y != to.y { return false; },
							pin::POS => if (to - from).turn45().x != 0 { return false; },
							pin::NEG => if (to - from).turn45().y != 0 { return false; },
							_ => {}
						}
					}
					Pos::RESERVE => return false,
					_ => {
						if self.pinned(from, !owner, self.kpos[owner as usize]) != pin::NO { return false; }
						if to != cpos && !to.between(cpos, self.kpos[owner as usize]) { return false; }
					}
				}
				
				if prmt {
					if CAN_PRMT[agent as usize][to.y as usize]
						| CAN_PRMT[agent as usize][from.y as usize] == 0 {return false}
				}
				else if agent & 8 == 0 {
					if CAN_PRMT[agent as usize][to.y as usize]
						| CAN_PRMT[agent as usize][from.y as usize] == 3 {return false}
				}
			}
		}
		else {
			let (owner, agent) = match from.x {
				9 => (player::BLACK, from.y as u8),
				10 => (player::WHITE, from.y as u8 | 0x10),
				_ => return false,
			};
			let 0 .. 7 = from.y else {return false};
			
			if self.turn != owner {return false}
			if self.rsv[owner as usize][from.y as usize] == 0 {return false}
			if self.terra[to.y as usize][to.x as usize] != 0 {return false}
			
			let cpos = self.checker();
			match cpos {
				Pos::REMOVED => {}
				Pos::RESERVE => return false,
				_ => if !to.between(cpos, self.kpos[owner as usize]) {
					return false;
				},
			}
			
			if prmt {return false}
			if CAN_PRMT[agent as usize][to.y as usize] == 3 {return false}
			if from.y as usize == pid::PAWN {
				for y in 0 .. 9 {
					if self.terra[y][to.x as usize] == agent + 1 {
						return false;
					}
				}
				if self.pawn_drop_mate(agent, to) {return false}
			}
		}
		return true;
	}
	
	pub fn for_attackers<T, F:FnMut(Pos, u8)->Option<T>>
		(&self, agrsor:bool, pos:Pos, mut f:F)->Option<T> {
		let mut apos;
		let mut grid;
		find_attacker!(agrsor, self, pos, apos, grid, {
			let ret = f(apos, grid);
			if ret.is_some() {return ret}
		});
		return None;
	}
	
	pub fn for_targets<T, F:FnMut(Pos, u8)->Option<T>>
		(&self, pos:Pos, mut f:F)->Option<T> {
		let grid = self.terra[pos.y as usize][pos.x as usize];
		if grid > 0 {
			for &diff in LIST_MOVE_MELEE[(grid - 1) as usize] {
				let t = pos + diff;
				let Pos{x:0 .. 9, y:0 .. 9} = t else {continue};
				let tg = self.terra[t.y as usize][t.x as usize];
				let ret = f(t, tg);
				if ret.is_some() {return ret}
			}
			'a:for &diff in LIST_MOVE_RANGED[(grid - 1) as usize] {
				let mut pos = pos;
				let mut grid;
				search_ray!(diff, self, pos, grid, {
					let ret = f(pos, grid);
					if ret.is_some() {return ret}
					if grid > 0 {continue 'a}
				});
			}
		}
		return None;
	}

	pub fn checks(&self)->bool {
		let kpos = self.kpos[self.turn as usize];
		if kpos == Pos::REMOVED {return false}
		find_attacker!(!self.turn, self, kpos, {return true});
		return false;
	}
	
	pub fn will_check(&self, mv:Move)->bool {
		let Move{from, to, prmt} = mv;
		let kpos = self.kpos[!self.turn as usize];
		
		let agent = match from.x {
			0 .. 9 => {
				match self.pinned(from, self.turn, kpos) {
					pin::HRZ => if from.y != to.y { return true; },
					pin::VRT => if from.x != to.x { return true; },
					pin::POS => if from.x - from.y != to.x - to.y { return true; },
					pin::NEG => if from.x + from.y != to.x + to.y { return true; },
					_ => {},
				}
				
				let a = self.terra[from.y as usize][from.x as usize] - 1;
				if prmt {a | 8} else {a}
			}
			9 => from.y as u8,
			10 => (from.y | 16) as u8,
			_ => return false,
		};
		let diff = kpos - to;
		return match CAN_MOVE[(agent & 0x1F) as usize][(diff.x + 8) as usize][(diff.y + 8) as usize] {
			1 => true,
			2 => {
				let mut pos = kpos;
				let mut grid;
				search_seg!(self, pos, to, grid, {return false});
				{true}
			}
			_ => false,
		};
	}
	
	fn checker(&self)->Pos {
		let kpos = self.kpos[self.turn as usize];
		let mut cpos = Pos::REMOVED;
		let Pos{x:0 .. 9, y:0 .. 9} = kpos else {return cpos};
		let mut apos;
		let mut grid;
		find_attacker!(!self.turn, self, kpos, apos, grid, {
			if cpos == Pos::REMOVED {
				cpos = apos;
			}
			else { return Pos::RESERVE; }
		});
		return cpos;
	}
	
	fn pawn_drop_mate(&self, pawn:u8, to:Pos)->bool {
		let owner = pawn & 0x10 != 0;
		let kpos = self.kpos[!owner as usize];
		let Pos{x:0 .. 9, y:0 .. 9} = kpos else {return false};
		let diff = kpos - to;
		if CAN_MOVE[pawn as usize][(diff.x + 8) as usize][(diff.y + 8) as usize] != 1 {return false}
		'a:for &kflee in LIST_MOVE_MELEE[(!owner as usize) << 4 | pid::KING] {
			let fpos = kpos + kflee;
			let Pos{x:0 ..= 8, y:0 ..= 8} = fpos else {continue};
			let iflee = self.terra[fpos.y as usize][fpos.x as usize];
			if iflee > 0 && (iflee - 1 & 0x10 != 0) != owner {continue}
			find_attacker!(owner, self, fpos, {continue 'a});
			return false;
		}
		
		let mut apos;
		let mut grid;
		find_attacker!(!owner, self, to, apos, grid, {
			if apos != kpos && self.pinned(apos, owner, kpos) == pin::NO {return false}
		});
		return true;
	}
}

pub use iter::NaiveUIter;

mod init;
mod iter;