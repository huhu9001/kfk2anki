use crate::board::Board;
use crate::board::Pos;
use crate::board::pid;
use crate::board::player;
use crate::rand::RandGen;

impl Board {
	pub fn random_monoking<RGen:RandGen<usize>>(rgen:&mut RGen)->Self {
		let mut b = Self::default();
		
		let kpos = Pos{
			x:(rgen.next_int() % 9) as i8,
			y:(rgen.next_int() % 3) as i8,
		};
		b.terra[kpos.y as usize][kpos.x as usize] = pid::grid(pid::KING, player::WHITE, false);
		b.kpos[player::WHITE as usize] = kpos;
		
		const NUM:[usize; 7] = [4, 4, 4, 4, 18, 2, 2];
		let mut pawn_used = [false; 18];
		for n in [pid::GUARD, pid::DOG, pid::ARCHER, pid::RAM, pid::CHARIOT, pid::HORSE] {
			for nn in 0 .. NUM[n] {
				if n == pid::PAWN {
					if rgen.next_int() % 3 != 0 {continue}
					else {pawn_used[nn] = true}
				}
				
				if rgen.next_int() % 2 == 0 {
					//attackers
					if rgen.next_int() % 3 == 0 {b.rsv[player::BLACK as usize][n] += 1}
					else {
						let pos = Pos{
							x:(rgen.next_int() % 9) as i8,
							y:(rgen.next_int() % 9) as i8,
						};
						let grid = &mut b.terra[pos.y as usize][pos.x as usize];
						if *grid == 0 {
							match n {
								pid::GUARD => *grid = pid::grid(n, player::BLACK, false),
								pid::PAWN =>
									if pos.y <= 0 || pos.y <= 2 && rgen.next_int() % 3 != 0 {
										*grid = pid::grid(n, player::BLACK, true);
									} else {b.rsv[player::BLACK as usize][n] += 1},
								pid::RAM =>
									*grid = pid::grid(n, player::BLACK, pos.y <= 0 || pos.y <= 2 && rgen.next_int() % 3 != 0),
								pid::ARCHER =>
									*grid = pid::grid(n, player::BLACK, pos.y <= 1 || pos.y <= 2 && rgen.next_int() % 3 != 0),
								_ =>
									*grid = pid::grid(n, player::BLACK, pos.y <= 2 && rgen.next_int() % 3 != 0),
							}
							
							if b.for_targets(pos, |_, grid| {
								return if grid == pid::grid(pid::KING, player::WHITE, false) {Some(())} else {None};
							}).is_some() {
								b.terra[pos.y as usize][pos.x as usize] = 0;
								b.rsv[player::BLACK as usize][n] += 1;
							}
						}
						else {b.rsv[player::BLACK as usize][n] += 1}
					}
				}
				else {
					//defenders
					if rgen.next_int() % 3 == 0 {b.rsv[player::WHITE as usize][n] += 1}
					else {
						let pos = if if let pid::GUARD | pid::DOG = n {true} else {false}
							&& rgen.next_int() % 2 == 0 {
							let mut gpos = kpos + match rgen.next_int() % 8 {
								0 => Pos{x:0, y:1},
								1 => Pos{x:1, y:1},
								2 => Pos{x:1, y:0},
								3 => Pos{x:1, y:-1},
								4 => Pos{x:0, y:-1},
								5 => Pos{x:-1, y:-1},
								6 => Pos{x:-1, y:0},
								_ => Pos{x:-1, y:1},
							};
							if gpos.x < 0 { gpos.x = 1; }
							else if gpos.x > 8 { gpos.x = 7; }
							if gpos.y < 0 { gpos.y = 1; }
							else if gpos.y > 8 { gpos.y = 7; }
							{gpos}
						} else {Pos{
							x:(rgen.next_int() % 9) as i8,
							y:(rgen.next_int() % 9) as i8,
						}};
						
						let grid = &mut b.terra[pos.y as usize][pos.x as usize];
						if *grid == 0 {
							match n {
								pid::GUARD => *grid = pid::grid(n, player::WHITE, false),
								pid::PAWN =>
									if pos.y >= 8 || pos.y >= 6 && rgen.next_int() % 3 != 0 {
										*grid = pid::grid(n, player::WHITE, true);
									} else {b.rsv[player::WHITE as usize][n] += 1},
								pid::RAM =>
									*grid = pid::grid(n, player::WHITE, pos.y >= 8 || pos.y >= 6 && rgen.next_int() % 3 != 0),
								pid::ARCHER =>
									*grid = pid::grid(n, player::WHITE, pos.y >= 7 || pos.y >= 6 && rgen.next_int() % 3 != 0),
								_ =>
									*grid = pid::grid(n, player::WHITE, pos.y >= 6 && rgen.next_int() % 3 != 0),
							}
						}
						else {b.rsv[player::WHITE as usize][n] += 1}
					}
				}
			}
		}
		
		//pawns
		for n in 0 .. 18 {
			if pawn_used[n] {continue}
			fn pawn_y(input:usize)->i8 {
				return match input % 81 {
					0 => 6,
					1 => 7,
					2 .. 5 => 0,
					5 .. 8 => 1,
					8 .. 17 => 4,
					17 .. 26 => 5,
					26 .. 53 => 3,
					_ => 2,
				};
			}
			
			if n % 2 == 0 {
				let mut pos = Pos{
					x:n as i8 / 2,
					y:8 - pawn_y(rgen.next_int()),
				};
				loop {
					if b.terra[pos.y as usize - 1][pos.x as usize] != pid::grid(pid::KING, player::WHITE, false) {
						let grid = &mut b.terra[pos.y as usize][pos.x as usize];
						if *grid == 0 {
							*grid = pid::grid(pid::PAWN, player::BLACK, false);
							break;
						}
					}
					pos.y -= 1;
					if pos.y <= 0 {
						b.rsv[player::BLACK as usize][pid::PAWN] += 1;
						break;
					}
				}
			}
			else {
				let mut pos = Pos{
					x:n as i8 / 2,
					y:pawn_y(rgen.next_int()),
				};
				loop {
					let grid = &mut b.terra[pos.y as usize][pos.x as usize];
					if *grid == 0 {
						*grid = pid::grid(pid::PAWN, player::WHITE, false);
						break;
					}
					pos.y += 1;
					if pos.y >= 8 {
						b.rsv[player::WHITE as usize][pid::PAWN] += 1;
						break;
					}
				}
			}
		}
		
		return b;
	}
}