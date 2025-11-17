pub mod player {
	pub const BLACK:bool = false;
	pub const WHITE:bool = true;
}

pub mod pid {
	pub const GUARD:usize = 0;
	pub const DOG:usize = 1;
	pub const ARCHER:usize = 2;
	pub const RAM:usize = 3;
	pub const PAWN:usize = 4;
	pub const CHARIOT:usize = 5;
	pub const HORSE:usize = 6;
	pub const KING:usize = 7;
	
	pub const fn grid(id:usize, owner:bool, prmt:bool)->u8 {
		return ((owner as u8) << 4 | (prmt as u8) << 3 | (id as u8)) + 1;
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pos {
	pub x:i8,
	pub y:i8,
}
impl Pos {
	pub const RESERVE:Self = Self{x:9, y:18};
	pub const REMOVED:Self = Self{x:19, y:9};
	
	pub const fn back(self, player:bool)->Self {
		return if player {self} else {Self{x:self.x, y:-self.y}};
	}
	
	pub const fn invert(self, player:bool)->Self {
		return if player {self} else {Self{x:8 - self.x, y:8 - self.y}};
	}
	
	pub const fn turn45(self)->Self {
		//rotating direction: cross product of x and y axes
		return Self{x:self.x - self.y, y:self.x + self.y};
	}
	
	pub const fn between(self, p1:Self, p2:Self)->bool {
		let dx1 = (p1.x - self.x) as i32;
		let dx2 = (p2.x - self.x) as i32;
		let dy1 = (p1.y - self.y) as i32;
		let dy2 = (p2.y - self.y) as i32;
		return
			dx1 * dy2 == dy1 * dx2 && dx1 * dx2 + dy1 * dy2 < 0;
	}
}
impl std::ops::Add for Pos {
	type Output = Self;
	fn add(self, other:Self)->Self {
		return Self{x:self.x + other.x, y:self.y + other.y };
	}
}
impl std::ops::Sub for Pos {
	type Output = Self;
	fn sub(self, other:Self)->Self {
		return Self{x:self.x - other.x, y:self.y - other.y };
	}
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Move {
	pub from:Pos,
	pub to:Pos,
	pub prmt:bool,
}

#[derive(Clone, Copy)] pub struct Unmove {
	pub mv:Move,
	pub victim:u8,
}

#[derive(Clone, Eq)] pub struct Board {
	pub turn:bool,
	pub terra:[[u8; 9]; 9],
	pub rsv:[[u8; 7]; 2],
	pub kpos:[Pos; 2],
}
impl Default for Board {
	fn default()->Self {
		return Self{
			turn:player::BLACK,
			terra:[[0; 9]; 9],
			rsv:[[0; 7]; 2],
			kpos:[Pos::REMOVED; 2],
		};
	}
}
impl Board {
	pub const NEW_STANDARD:&str = "|\
	RADGKGDAR\
	.C.....H.\
	PPPPPPPPP\
	.........\
	.........\
	.........\
	ppppppppp\
	.h.....c.\
	radgkgdar\
	|";

	pub fn from_parts(turn:bool, terra:[[u8; 9]; 9], rsv:[[u8; 7]; 2])->Self {
		let mut b = Self{turn, terra, rsv, kpos:[Pos::REMOVED; 2]};
		for y in 0 .. 9 {
			for x in 0 .. 9 {
				if b.terra[y as usize][x as usize] == pid::grid(pid::KING, player::BLACK, false) {
					b.kpos[player::BLACK as usize] = Pos{x, y};
				}
				else if b.terra[y as usize][x as usize] == pid::grid(pid::KING, player::WHITE, false) {
					b.kpos[player::WHITE as usize] = Pos{x, y};
				}
			}
		}
		return b;
	}
	
	pub fn do_move(&mut self, mv:Move)->Unmove {
		let Move{from, to, prmt} = mv;
		self.turn ^= true;
		match from.x {
			0 .. 9 => {
				let gf = &mut self.terra[from.y as usize][from.x as usize];
				let agent = if prmt {*gf | 8} else {*gf};
				*gf = 0;
				
				let gt = &mut self.terra[to.y as usize][to.x as usize];
				let victim = *gt;
				*gt = agent;
				
				if agent - 1 & 7 == pid::KING as u8 {
					self.kpos[((agent - 1 & 16) >> 4) as usize] = to;
				}
				
				if victim > 0 {
					self.rsv[((agent - 1 & 16) >> 4) as usize][(victim - 1 & 7) as usize] += 1;
				}
				return Unmove{mv, victim};
			}
			9 => {
				self.rsv[player::BLACK as usize][from.y as usize] -= 1;
				self.terra[to.y as usize][to.x as usize] = pid::grid(from.y as usize, player::BLACK, false);
				return Unmove{mv, victim:0};
			}
			10 => {
				self.rsv[player::WHITE as usize][from.y as usize] -= 1;
				self.terra[to.y as usize][to.x as usize] = pid::grid(from.y as usize, player::WHITE, false);
				return Unmove{mv, victim:0};
			}
			_ => return Unmove{mv, victim:0},
		}
	}

	pub fn undo_move(&mut self, umv:Unmove) {
		let Unmove{mv:Move{from, to, prmt}, victim} = umv;
		self.turn ^= true;
		match from.x {
			0 .. 9 => {
				let gt = &mut self.terra[to.y as usize][to.x as usize];
				let agent = if prmt {*gt & !8} else {*gt};
				*gt = victim;
				
				self.terra[from.y as usize][from.x as usize] = agent;
				
				if agent - 1 & 7 == pid::KING as u8 {
					self.kpos[((agent - 1 & 16) >> 4) as usize] = from;
				}
				
				if victim > 0 {
					self.rsv[((agent - 1 & 16) >> 4) as usize][(victim - 1 & 7) as usize] -= 1;
				}
			}
			9 => {
				self.rsv[player::BLACK as usize][from.y as usize] += 1;
				self.terra[to.y as usize][to.x as usize] = 0;
			}
			10 => {
				self.rsv[player::WHITE as usize][from.y as usize] += 1;
				self.terra[to.y as usize][to.x as usize] = 0;
			}
			_ => {}
		}
	}

	pub fn invert(&mut self) {
		self.turn ^= true;
		self.terra.reverse();
		for row in &mut self.terra {
			row.reverse();
			for grid in row {
				let g = *grid;
				if g > 0 {
					*grid = (g - 1 ^ 16) + 1;
				}
			}
		}
		self.rsv.swap(0, 1);
		self.kpos.swap(0, 1);
		for kp in &mut self.kpos {
			kp.x = 8 - kp.x;
			kp.y = 8 - kp.y;
		}
	}
}
impl PartialEq for Board {
	fn eq(&self, other:&Self)-> bool {
		return self.turn == other.turn
			&& self.terra == other.terra
			&& self.rsv[0] == self.rsv[0];
	}
}

pub mod io;