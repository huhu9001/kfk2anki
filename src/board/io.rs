use crate::board::Board;
use crate::board::Pos;
use crate::board::pid;
use crate::board::player;

use std::fmt::Write;

const P_JP:[char; 16] = [
	'金', '銀', '桂', '香', '歩', '飛', '角', '王',
	'金', '全', '圭', '杏', 'と', '龍', '馬', '王',
];

impl From<&str> for Board {
	fn from(s:&str)->Self {
		let mut b = Self::default();
		let mut in_reserve = 0;
		let mut pos = Pos{x:8, y:0};
		for c in s.as_bytes() {
			match c {
				b' ' | b'\r' | b'\n' | b'\t' => {}
				b'|' => in_reserve += 1,
				b'!' => b.turn ^= true,
				_ => match in_reserve {
					1 => {
						let grid = &mut b.terra[pos.y as usize][pos.x as usize];
						match c {
							b'k' => {
								*grid = pid::grid(pid::KING, player::BLACK, false);
								b.kpos[player::BLACK as usize] = pos;
							}
							b'K' => {
								*grid = pid::grid(pid::KING, player::WHITE, false);
								b.kpos[player::WHITE as usize] = pos;
							}
							b'g' => *grid = pid::grid(pid::GUARD, player::BLACK, false),
							b'G' => *grid = pid::grid(pid::GUARD, player::WHITE, false),
							b'd' => *grid = pid::grid(pid::DOG, player::BLACK, false),
							b'D' => *grid = pid::grid(pid::DOG, player::WHITE, false),
							b'w' => *grid = pid::grid(pid::DOG, player::BLACK, true),
							b'W' => *grid = pid::grid(pid::DOG, player::WHITE, true),
							b'a' => *grid = pid::grid(pid::ARCHER, player::BLACK, false),
							b'A' => *grid = pid::grid(pid::ARCHER, player::WHITE, false),
							b'x' => *grid = pid::grid(pid::ARCHER, player::BLACK, true),
							b'X' => *grid = pid::grid(pid::ARCHER, player::WHITE, true),
							b'r' => *grid = pid::grid(pid::RAM, player::BLACK, false),
							b'R' => *grid = pid::grid(pid::RAM, player::WHITE, false),
							b'y' => *grid = pid::grid(pid::RAM, player::BLACK, true),
							b'Y' => *grid = pid::grid(pid::RAM, player::WHITE, true),
							b'p' => *grid = pid::grid(pid::PAWN, player::BLACK, false),
							b'P' => *grid = pid::grid(pid::PAWN, player::WHITE, false),
							b'z' => *grid = pid::grid(pid::PAWN, player::BLACK, true),
							b'Z' => *grid = pid::grid(pid::PAWN, player::WHITE, true),
							b'c' => *grid = pid::grid(pid::CHARIOT, player::BLACK, false),
							b'C' => *grid = pid::grid(pid::CHARIOT, player::WHITE, false),
							b'u' => *grid = pid::grid(pid::CHARIOT, player::BLACK, true),
							b'U' => *grid = pid::grid(pid::CHARIOT, player::WHITE, true),
							b'h' => *grid = pid::grid(pid::HORSE, player::BLACK, false),
							b'H' => *grid = pid::grid(pid::HORSE, player::WHITE, false),
							b'v' => *grid = pid::grid(pid::HORSE, player::BLACK, true),
							b'V' => *grid = pid::grid(pid::HORSE, player::WHITE, true),
							_ => {}
						}
						if pos.x > 0 { pos.x -= 1 }
						else {
							pos.x = 8;
							pos.y += 1;
							if pos.y >= 9 { pos.y = 0; }
						}
					}
					0 => match c {
						b'g' | b'G' => b.rsv[player::WHITE as usize][pid::GUARD] += 1,
						b'd' | b'D' => b.rsv[player::WHITE as usize][pid::DOG] += 1,
						b'a' | b'A' => b.rsv[player::WHITE as usize][pid::ARCHER] += 1,
						b'r' | b'R' => b.rsv[player::WHITE as usize][pid::RAM] += 1,
						b'p' | b'P' => b.rsv[player::WHITE as usize][pid::PAWN] += 1,
						b'c' | b'C' => b.rsv[player::WHITE as usize][pid::CHARIOT] += 1,
						b'h' | b'H' => b.rsv[player::WHITE as usize][pid::HORSE] += 1,
						_ => {}
					}
					2 => match c {
						b'g' | b'G' => b.rsv[player::BLACK as usize][pid::GUARD] += 1,
						b'd' | b'D' => b.rsv[player::BLACK as usize][pid::DOG] += 1,
						b'a' | b'A' => b.rsv[player::BLACK as usize][pid::ARCHER] += 1,
						b'r' | b'R' => b.rsv[player::BLACK as usize][pid::RAM] += 1,
						b'p' | b'P' => b.rsv[player::BLACK as usize][pid::PAWN] += 1,
						b'c' | b'C' => b.rsv[player::BLACK as usize][pid::CHARIOT] += 1,
						b'h' | b'H' => b.rsv[player::BLACK as usize][pid::HORSE] += 1,
						_ => {}
					}
					_ => {}
				},
			}
		}
		return b;
	}
}

impl std::str::FromStr for Board {
	type Err = isize;
	fn from_str(s:&str)->Result<Self, Self::Err> {
		let s = s.trim_start();
		match s.chars().next() {
			Some('[') => {
				//PSN
				let Some(start) = s.find("[SFEN") else {return Err(-1)};
				let s = &s[start + 5..].trim_start();
				
				let Some(&b'"') = s.as_bytes().get(0) else {return Err(-2)};
				let s = &s[1..];
				
				let mut b = Self::default();
				let mut x = 8;
				let mut y = 0;
				let mut prmt = false;
				let mut index = 0;
				for c in s.as_bytes() {
					index += 1;
					match c {
						b' ' => break,
						b'1' ..= b'9' => x -= (c - b'0') as i8,
						b'/' => {x = 8; y += 1}
						b'+' => prmt = true,
						_ => {
							let Some(line) = b.terra.get_mut(y as usize) else {return Err(-3)};
							let Some(grid) = line.get_mut(x as usize) else {return Err(-4)};
							macro_rules! p {
								($id:expr, b) => {
									*grid = pid::grid($id, player::BLACK, prmt);
									prmt = false;
								};
								($id:expr, w) => {
									*grid = pid::grid($id, player::WHITE, prmt);
									prmt = false;
								};
							}
							match c {
								b'K' =>
									if prmt {return Err(-5)}
									else {
										*grid = pid::grid(pid::KING, player::BLACK, false);
										b.kpos[player::BLACK as usize] = Pos{x, y};
									},
								b'k' =>
									if prmt {return Err(-6)}
									else {
										*grid = pid::grid(pid::KING, player::WHITE, false);
										b.kpos[player::WHITE as usize] = Pos{x, y};
									},
								b'G' =>
									if prmt {return Err(-7)}
									else {*grid = pid::grid(pid::GUARD, player::BLACK, false)},
								b'g' =>
									if prmt {return Err(-8)}
									else {*grid = pid::grid(pid::GUARD, player::WHITE, false)},
								b'S' => { p!(pid::DOG, b); }
								b's' => { p!(pid::DOG, w); }
								b'N' => { p!(pid::ARCHER, b); }
								b'n' => { p!(pid::ARCHER, w); }
								b'L' => { p!(pid::RAM, b); }
								b'l' => { p!(pid::RAM, w); }
								b'P' => { p!(pid::PAWN, b); }
								b'p' => { p!(pid::PAWN, w); }
								b'R' => { p!(pid::CHARIOT, b); }
								b'r' => { p!(pid::CHARIOT, w); }
								b'B' => { p!(pid::HORSE, b); }
								b'b' => { p!(pid::HORSE, w); }
								_ => return Err(-9),
							}
							x -= 1;
						}
					}
				}
				
				let s = &s[index..].trim_ascii_start();
				b.turn = match s.as_bytes().get(0) {
					Some(&b'b') => false,
					Some(&b'w') => true,
					_ => return Err(-10),
				};
				
				let s = &s[1..];
				if let Some(&b' ') = s.as_bytes().get(0) {
					let s = s.trim_ascii_start();
					let mut num = 1;
					let mut numming = false;
					for c in s.as_bytes() {
						match c {
							b'0' ..= b'9' => {
								num = c - b'0' + if numming {num * 10} else {0};
								numming = true;
								continue;
							}
							b'G' => b.rsv[player::BLACK as usize][pid::GUARD] += num,
							b'g' => b.rsv[player::WHITE as usize][pid::GUARD] += num,
							b'S' => b.rsv[player::BLACK as usize][pid::DOG] += num,
							b's' => b.rsv[player::WHITE as usize][pid::DOG] += num,
							b'N' => b.rsv[player::BLACK as usize][pid::ARCHER] += num,
							b'n' => b.rsv[player::WHITE as usize][pid::ARCHER] += num,
							b'L' => b.rsv[player::BLACK as usize][pid::RAM] += num,
							b'l' => b.rsv[player::WHITE as usize][pid::RAM] += num,
							b'P' => b.rsv[player::BLACK as usize][pid::PAWN] += num,
							b'p' => b.rsv[player::WHITE as usize][pid::PAWN] += num,
							b'R' => b.rsv[player::BLACK as usize][pid::CHARIOT] += num,
							b'r' => b.rsv[player::WHITE as usize][pid::CHARIOT] += num,
							b'B' => b.rsv[player::BLACK as usize][pid::HORSE] += num,
							b'b' => b.rsv[player::WHITE as usize][pid::HORSE] += num,
							_ => break,
						}
						num = 1;
						numming = false;
					}
				}
				
				return Ok(b);
			}
			Some('N') => {
				//CSA
				todo!{}
			}
			Some('後') => {
				//KIF
				todo!{}
			}
			_ => return Ok(Self::from(s)),
		}
	}
}

impl std::fmt::Display for Board {
	fn fmt(&self, f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {
		writeln!(f, "[Sente \"\"]")?;
		writeln!(f, "[Gote \"\"]")?;
		return write!(f, "[SFEN \"{}\"]", SFEN(self));
	}
}

pub struct SFEN<'a>(pub &'a Board);
impl std::fmt::Display for SFEN<'_> {
	fn fmt(&self, f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {
		const fn init_c()->[&'static str; 0x20] {
			let mut r = [""; 0x20];
			
			r[pid::grid(pid::KING, player::BLACK, false) as usize - 1] = "K";
			r[pid::grid(pid::KING, player::WHITE, false) as usize - 1] = "k";
			r[pid::grid(pid::GUARD, player::BLACK, false) as usize - 1] = "G";
			r[pid::grid(pid::GUARD, player::WHITE, false) as usize - 1] = "g";
			r[pid::grid(pid::DOG, player::BLACK, false) as usize - 1] = "S";
			r[pid::grid(pid::DOG, player::WHITE, false) as usize - 1] = "s";
			r[pid::grid(pid::DOG, player::BLACK, true) as usize - 1] = "+S";
			r[pid::grid(pid::DOG, player::WHITE, true) as usize - 1] = "+s";
			r[pid::grid(pid::ARCHER, player::BLACK, false) as usize - 1] = "N";
			r[pid::grid(pid::ARCHER, player::WHITE, false) as usize - 1] = "n";
			r[pid::grid(pid::ARCHER, player::BLACK, true) as usize - 1] = "+N";
			r[pid::grid(pid::ARCHER, player::WHITE, true) as usize - 1] = "+n";
			r[pid::grid(pid::RAM, player::BLACK, false) as usize - 1] = "L";
			r[pid::grid(pid::RAM, player::WHITE, false) as usize - 1] = "l";
			r[pid::grid(pid::RAM, player::BLACK, true) as usize - 1] = "+L";
			r[pid::grid(pid::RAM, player::WHITE, true) as usize - 1] = "+l";
			r[pid::grid(pid::PAWN, player::BLACK, false) as usize - 1] = "P";
			r[pid::grid(pid::PAWN, player::WHITE, false) as usize - 1] = "p";
			r[pid::grid(pid::PAWN, player::BLACK, true) as usize - 1] = "+P";
			r[pid::grid(pid::PAWN, player::WHITE, true) as usize - 1] = "+p";
			r[pid::grid(pid::CHARIOT, player::BLACK, false) as usize - 1] = "R";
			r[pid::grid(pid::CHARIOT, player::WHITE, false) as usize - 1] = "r";
			r[pid::grid(pid::CHARIOT, player::BLACK, true) as usize - 1] = "+R";
			r[pid::grid(pid::CHARIOT, player::WHITE, true) as usize - 1] = "+r";
			r[pid::grid(pid::HORSE, player::BLACK, false) as usize - 1] = "B";
			r[pid::grid(pid::HORSE, player::WHITE, false) as usize - 1] = "b";
			r[pid::grid(pid::HORSE, player::BLACK, true) as usize - 1] = "+B";
			r[pid::grid(pid::HORSE, player::WHITE, true) as usize - 1] = "+b";
			
			return r;
		}
		const C:[&'static str; 0x20] = init_c();

		
		let &Self(b) = self;
			
		let mut s_terra = String::new();
		for y in 0 .. 9 {
			let mut ne = 0;
			for x in (0 .. 9).rev() {
				let grid = b.terra[y][x];
				if grid > 0 {
					if ne > 0 {
						s_terra.push(char::from(b'0' + ne));
						ne = 0;
					}
					s_terra.push_str(C[(grid - 1 & 0x1F) as usize]);
					continue;
				}
				ne += 1;
			}
			if ne > 0 {s_terra.push(char::from(b'0' + ne))}
			s_terra.push('/');
		}
		s_terra.pop();
		
		let mut s_rsv = String::new();
		for pl in [player::BLACK, player::WHITE] {
			for n in 0 .. 7 {
				let num = b.rsv[pl as usize][n] as usize;
				if num > 0 {
					if num > 1 {write!(&mut s_rsv, "{num}")?}
					s_rsv.push_str(C[pid::grid(n, pl, false) as usize - 1]);
				}
			}
		}
		
		return write!(f, "{s_terra} {} {s_rsv} 1", if b.turn {'w'} else {'b'});
	}
}

pub struct H<'a>(pub &'a Board);
impl std::fmt::Display for H<'_> {
	fn fmt(&self, f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {
		const fn init_c()->[u8; 32] {
			let mut r = [0; 32];
			
			r[pid::grid(pid::KING, player::BLACK, false) as usize - 1] = b'k';
			r[pid::grid(pid::KING, player::WHITE, false) as usize - 1] = b'K';
			r[pid::grid(pid::GUARD, player::BLACK, false) as usize - 1] = b'g';
			r[pid::grid(pid::GUARD, player::WHITE, false) as usize - 1] = b'G';
			r[pid::grid(pid::DOG, player::BLACK, false) as usize - 1] = b'd';
			r[pid::grid(pid::DOG, player::WHITE, false) as usize - 1] = b'D';
			r[pid::grid(pid::DOG, player::BLACK, true) as usize - 1] = b'w';
			r[pid::grid(pid::DOG, player::WHITE, true) as usize - 1] = b'W';
			r[pid::grid(pid::ARCHER, player::BLACK, false) as usize - 1] = b'a';
			r[pid::grid(pid::ARCHER, player::WHITE, false) as usize - 1] = b'A';
			r[pid::grid(pid::ARCHER, player::BLACK, true) as usize - 1] = b'x';
			r[pid::grid(pid::ARCHER, player::WHITE, true) as usize - 1] = b'X';
			r[pid::grid(pid::RAM, player::BLACK, false) as usize - 1] = b'r';
			r[pid::grid(pid::RAM, player::WHITE, false) as usize - 1] = b'R';
			r[pid::grid(pid::RAM, player::BLACK, true) as usize - 1] = b'y';
			r[pid::grid(pid::RAM, player::WHITE, true) as usize - 1] = b'Y';
			r[pid::grid(pid::PAWN, player::BLACK, false) as usize - 1] = b'p';
			r[pid::grid(pid::PAWN, player::WHITE, false) as usize - 1] = b'P';
			r[pid::grid(pid::PAWN, player::BLACK, true) as usize - 1] = b'z';
			r[pid::grid(pid::PAWN, player::WHITE, true) as usize - 1] = b'Z';
			r[pid::grid(pid::CHARIOT, player::BLACK, false) as usize - 1] = b'c';
			r[pid::grid(pid::CHARIOT, player::WHITE, false) as usize - 1] = b'C';
			r[pid::grid(pid::CHARIOT, player::BLACK, true) as usize - 1] = b'u';
			r[pid::grid(pid::CHARIOT, player::WHITE, true) as usize - 1] = b'U';
			r[pid::grid(pid::HORSE, player::BLACK, false) as usize - 1] = b'h';
			r[pid::grid(pid::HORSE, player::WHITE, false) as usize - 1] = b'H';
			r[pid::grid(pid::HORSE, player::BLACK, true) as usize - 1] = b'v';
			r[pid::grid(pid::HORSE, player::WHITE, true) as usize - 1] = b'V';
			
			return r;
		}
		const C:[u8; 32] = init_c();
		
		let &Self(b) = self;
		
		let mut str_board = String::from("|\n\
		.........\n\
		.........\n\
		.........\n\
		.........\n\
		.........\n\
		.........\n\
		.........\n\
		.........\n\
		.........\n\
		|");
		let mut str_reserve_black = String::with_capacity(40);
		let mut str_reserve_white = String::with_capacity(40);
		
		let sb = unsafe {str_board.as_bytes_mut()};
		for x in 0 .. 9 {
			for y in 0 .. 9 {
				let grid = b.terra[y][x];
				if grid > 0 {
					let c = C[(grid - 1 & 0x1F) as usize];
					if c > 0 {
						*unsafe {sb.get_unchecked_mut(y * 10 + 10 - x)} = c;
					}
				}
			}
		}
		
		for n in 0 .. 7 {
			for _ in 0 .. b.rsv[0][n] {
				str_reserve_black.push(char::from(C[n]));
			}
			for _ in 0 .. b.rsv[1][n] {
				str_reserve_white.push(char::from(C[n | 0x10]));
			}
		}
		
		return write!(f, "{str_reserve_white}{str_board}{str_reserve_black}{}", if b.turn {"!"} else {""});
	}
}

pub struct SVG<'a>(pub &'a Board);
impl std::fmt::Display for SVG<'_> {
	fn fmt(&self, f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {
		write!(f, "{}", r#"<svg viewBox="0 0 100 120" xmlns="http://www.w3.org/2000/svg"><style>path{stroke:black;fill:none}text{stroke:none;fill:black;font-size:6px}</style><rect width="100%" height="100%" fill="white"/><path d="M5 15h90v10h-90v10h90v10h-90v10h90v10h-90v10h90v10h-90v10h90v10h-90v-90h10v90h10v-90h10v90h10v-90h10v90h10v-90h10v90h10v-90h10v90"/>"#)?;

		let &Self(b) = self;

		//field pieces
		let mut p_black = String::new();
		let mut x_black = String::new();
		let mut y_black = String::new();
		let mut p_white = String::new();
		let mut x_white = String::new();
		let mut y_white = String::new();
		for y in 0 .. 9 {
			for x in 0 .. 9 {
				let g = b.terra[y][x];
				if g > 0 {
					let g = g - 1;
					if g & 0x10 == 0 {
						p_black.push(P_JP[g as usize & 0xF]);
						write!(x_black, "{},", (8 - x) * 10 + 7)?;
						write!(y_black, "{},", y * 10 + 22)?;
					}
					else {
						p_white.push(P_JP[g as usize & 0xF]);
						write!(x_white, "{},", (8 - x) * 10 + 13)?;
						write!(y_white, "{},", y * 10 + 18)?;
					}
				}
			}
		}
		if !p_black.is_empty() {
			x_black.pop();
			y_black.pop();
			write!(f, r#"<text x="{x_black}" y="{y_black}">{p_black}</text>"#)?;
		}
		if !p_white.is_empty() {
			x_white.pop();
			y_white.pop();
			write!(f, r#"<text rotate="180" x="{x_white}" y="{y_white}">{p_white}</text>"#)?;
		}

		//reserve pieces
		if let Some(k) = b.rsv[0].iter().position(|&it| it > 0) {
			write!(f, r#"<text x="5" y="112">"#)?;
			for k in k .. 7 {
				let num = b.rsv[0][k];
				if num > 0 {
					write!(f, "{}{num}", P_JP[k])?;
				}
			}
			write!(f, "</text>")?;
		}
		if let Some(k) = b.rsv[1].iter().position(|&it| it > 0) {
			write!(f, r#"<text x="5" y="12">"#)?;
			for k in k .. 7 {
				let num = b.rsv[1][k];
				if num > 0 {
					write!(f, "{}{num}", P_JP[k])?;
				}
			}
			write!(f, "</text>")?;
		}

		if b.turn {
			write!(f, r#"<text x="90" y="12" style="fill:red">後</text>"#)?;
		}

		return write!(f, "</svg>");
	}
}

#[cfg(any(debug_assertions, test))] mod debug {
	use crate::board::Board;
	use crate::board::Move;
	use crate::board::Pos;
use crate::board::io::H;
	
	impl std::fmt::Debug for Pos {
		fn fmt(&self, f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {
			return write!(f, "({},{})", self.x, self.y);
		}
	}
	
	impl std::fmt::Debug for Move {
		fn fmt(&self, f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {
			return write!(f, "{:?}{}{:?}", self.from, if self.prmt {"+"} else {"-"}, self.to);
		}
	}
	
	impl std::fmt::Debug for Board {
		fn fmt(&self, f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {
			return write!(f, "{}", H(self));
		}
	}
}