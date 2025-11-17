use super::board::{Board, Pos, pid};
use crate::search_ray;
use crate::moves::LIST_MOVE_MELEE;
use crate::moves::LIST_MOVE_RANGED;

const fn mat_rsv()->[i32; 0x20] {
	let mut r = [0; 0x20];
	
	r[pid::GUARD] = 540;
	r[pid::DOG] = 495;
	r[pid::DOG | 8] = r[pid::GUARD];
	r[pid::ARCHER] = 405;
	r[pid::ARCHER | 8] = r[pid::GUARD];
	r[pid::RAM] = 315;
	r[pid::RAM | 8] = r[pid::GUARD];
	r[pid::PAWN] = 90;
	r[pid::PAWN | 8] = r[pid::GUARD];
	r[pid::CHARIOT] = 990;
	r[pid::CHARIOT | 8] = 1390;
	r[pid::HORSE] = 855;
	r[pid::HORSE | 8] = 945;
	
	r[pid::GUARD | 0x10] = -r[pid::GUARD];
	r[pid::DOG | 0x10] = -r[pid::DOG];
	r[pid::DOG | 0x18] = -r[pid::DOG | 8];
	r[pid::ARCHER | 0x10] = -r[pid::ARCHER];
	r[pid::ARCHER | 0x18] = -r[pid::ARCHER | 8];
	r[pid::RAM | 0x10] = -r[pid::RAM];
	r[pid::RAM | 0x18] = -r[pid::RAM | 8];
	r[pid::PAWN | 0x10] = -r[pid::PAWN];
	r[pid::PAWN | 0x18] = -r[pid::GUARD];
	r[pid::CHARIOT | 0x10] = -r[pid::CHARIOT];
	r[pid::CHARIOT | 0x18] = -r[pid::CHARIOT | 8];
	r[pid::HORSE | 0x10] = -r[pid::HORSE];
	r[pid::HORSE | 0x18] = -r[pid::HORSE | 8];
	
	return r;
}
const fn mat_terra()->[i32; 0x20] {
	let mut r = mat_rsv();
	let mut i = 0;
	while i < 0x20 {
		r[i] = r[i] * (1024 - 104) / 1024;
		i += 1;
	}
	return r;
}
const MAT_RSV:[i32; 0x20] = mat_rsv();
const MAT_TERRA:[i32; 0x20] = mat_terra();

macro_rules! e_col {
	(1) => {6365.0 - 5341.0};
	(2) => {6365.0 - 5341.0 * 0.8525};
	(3) => {6365.0 - 5341.0 * 0.8525 * 0.8525};
	(4) => {6365.0 - 5341.0 * 0.8525 * 0.8525 * 0.8525};
	(5) => {6365.0 - 5341.0 * 0.8525 * 0.8525 * 0.8525 * 0.8525};
	(6) => {6365.0 - 5341.0 * 0.8525 * 0.8525 * 0.8525 * 0.8525 * 0.8525};
	(7) => {6365.0 - 5341.0 * 0.8525 * 0.8525 * 0.8525 * 0.8525 * 0.8525 * 0.8525};
	(8) => {6365.0 - 5341.0 * 0.8525 * 0.8525 * 0.8525 * 0.8525 * 0.8525 * 0.8525 * 0.8525};
	(9) => {6365.0 - 5341.0 * 0.8525 * 0.8525 * 0.8525 * 0.8525 * 0.8525 * 0.8525 * 0.8525 * 0.8525};
	(10) => {6365.0 - 5341.0 * 0.8525 * 0.8525 * 0.8525 * 0.8525 * 0.8525 * 0.8525 * 0.8525 * 0.8525 * 0.8525};
}
macro_rules! ef_row {
	($d:literal) => {85.0 * 1024.0 / ($d as f32 + 1.0)};
}
macro_rules! ee_row {
	($d:literal) => {98.0 * 1024.0 / ($d as f32 + 1.0)};
}
macro_rules! ef {
	(1, $y:literal) => {(e_col!(1) * ef_row!($y) / 1024.0 / 1024.0) as i32};
	(2, $y:literal) => {(e_col!(2) * ef_row!($y) / 1024.0 / 1024.0) as i32};
	(3, $y:literal) => {(e_col!(3) * ef_row!($y) / 1024.0 / 1024.0) as i32};
	(4, $y:literal) => {(e_col!(4) * ef_row!($y) / 1024.0 / 1024.0) as i32};
	(6, $y:literal) => {(e_col!(6) * ef_row!($y) / 1024.0 / 1024.0) as i32};
	(7, $y:literal) => {(e_col!(7) * ef_row!($y) / 1024.0 / 1024.0) as i32};
	(5, $y:literal) => {(e_col!(5) * ef_row!($y) / 1024.0 / 1024.0) as i32};
	(8, $y:literal) => {(e_col!(8) * ef_row!($y) / 1024.0 / 1024.0) as i32};
	(9, $y:literal) => {(e_col!(9) * ef_row!($y) / 1024.0 / 1024.0) as i32};
	(10, $y:literal) => {(e_col!(10) * ef_row!($y) / 1024.0 / 1024.0) as i32};
}
macro_rules! ee {
	(1, $y:literal) => {(e_col!(1) * ee_row!($y) / 1024.0 / 1024.0) as i32};
	(2, $y:literal) => {(e_col!(2) * ee_row!($y) / 1024.0 / 1024.0) as i32};
	(3, $y:literal) => {(e_col!(3) * ee_row!($y) / 1024.0 / 1024.0) as i32};
	(4, $y:literal) => {(e_col!(4) * ee_row!($y) / 1024.0 / 1024.0) as i32};
	(5, $y:literal) => {(e_col!(5) * ee_row!($y) / 1024.0 / 1024.0) as i32};
	(6, $y:literal) => {(e_col!(6) * ee_row!($y) / 1024.0 / 1024.0) as i32};
	(7, $y:literal) => {(e_col!(7) * ee_row!($y) / 1024.0 / 1024.0) as i32};
	(8, $y:literal) => {(e_col!(8) * ee_row!($y) / 1024.0 / 1024.0) as i32};
	(9, $y:literal) => {(e_col!(9) * ee_row!($y) / 1024.0 / 1024.0) as i32};
	(10, $y:literal) => {(e_col!(10) * ef_row!($y) / 1024.0 / 1024.0) as i32};
}
const EFF_FRIEND:[[i32; 9]; 11] = [
	[0; 9],
	[ef!(1, 0), ef!(1, 1), ef!(1, 2), ef!(1, 3), ef!(1, 4), ef!(1, 5), ef!(1, 6), ef!(1, 7), ef!(1, 8)],
	[ef!(2, 0), ef!(2, 1), ef!(2, 2), ef!(2, 3), ef!(2, 4), ef!(2, 5), ef!(2, 6), ef!(2, 7), ef!(2, 8)],
	[ef!(3, 0), ef!(3, 1), ef!(3, 2), ef!(3, 3), ef!(3, 4), ef!(3, 5), ef!(3, 6), ef!(3, 7), ef!(3, 8)],
	[ef!(4, 0), ef!(4, 1), ef!(4, 2), ef!(4, 3), ef!(4, 4), ef!(4, 5), ef!(4, 6), ef!(4, 7), ef!(4, 8)],
	[ef!(5, 0), ef!(5, 1), ef!(5, 2), ef!(5, 3), ef!(5, 4), ef!(5, 5), ef!(5, 6), ef!(5, 7), ef!(5, 8)],
	[ef!(6, 0), ef!(6, 1), ef!(6, 2), ef!(6, 3), ef!(6, 4), ef!(6, 5), ef!(6, 6), ef!(6, 7), ef!(6, 8)],
	[ef!(7, 0), ef!(7, 1), ef!(7, 2), ef!(7, 3), ef!(7, 4), ef!(7, 5), ef!(7, 6), ef!(7, 7), ef!(7, 8)],
	[ef!(8, 0), ef!(8, 1), ef!(8, 2), ef!(8, 3), ef!(8, 4), ef!(8, 5), ef!(8, 6), ef!(8, 7), ef!(8, 8)],
	[ef!(9, 0), ef!(9, 1), ef!(9, 2), ef!(9, 3), ef!(9, 4), ef!(9, 5), ef!(9, 6), ef!(9, 7), ef!(9, 8)],
	[ef!(10, 0), ef!(10, 1), ef!(10, 2), ef!(10, 3), ef!(10, 4), ef!(10, 5), ef!(10, 6), ef!(10, 7), ef!(10, 8)],
];
const EFF_ENEMY:[[i32; 9]; 11] = [
	[0; 9],
	[ee!(1, 0), ee!(1, 1), ee!(1, 2), ee!(1, 3), ee!(1, 4), ee!(1, 5), ee!(1, 6), ee!(1, 7), ee!(1, 8)],
	[ee!(2, 0), ee!(2, 1), ee!(2, 2), ee!(2, 3), ee!(2, 4), ee!(2, 5), ee!(2, 6), ee!(2, 7), ee!(2, 8)],
	[ee!(3, 0), ee!(3, 1), ee!(3, 2), ee!(3, 3), ee!(3, 4), ee!(3, 5), ee!(3, 6), ee!(3, 7), ee!(3, 8)],
	[ee!(4, 0), ee!(4, 1), ee!(4, 2), ee!(4, 3), ee!(4, 4), ee!(4, 5), ee!(4, 6), ee!(4, 7), ee!(4, 8)],
	[ee!(5, 0), ee!(5, 1), ee!(5, 2), ee!(5, 3), ee!(5, 4), ee!(5, 5), ee!(5, 6), ee!(5, 7), ee!(5, 8)],
	[ee!(6, 0), ee!(6, 1), ee!(6, 2), ee!(6, 3), ee!(6, 4), ee!(6, 5), ee!(6, 6), ee!(6, 7), ee!(6, 8)],
	[ee!(7, 0), ee!(7, 1), ee!(7, 2), ee!(7, 3), ee!(7, 4), ee!(7, 5), ee!(7, 6), ee!(7, 7), ee!(7, 8)],
	[ee!(8, 0), ee!(8, 1), ee!(8, 2), ee!(8, 3), ee!(8, 4), ee!(8, 5), ee!(8, 6), ee!(8, 7), ee!(8, 8)],
	[ee!(9, 0), ee!(9, 1), ee!(9, 2), ee!(9, 3), ee!(9, 4), ee!(9, 5), ee!(9, 6), ee!(9, 7), ee!(9, 8)],
	[ee!(10, 0), ee!(10, 1), ee!(10, 2), ee!(10, 3), ee!(10, 4), ee!(10, 5), ee!(10, 6), ee!(10, 7), ee!(10, 8)],
];
const V_KINGRANK:[i32; 9] = [2100, 1000, 600, 0, 0, 0, 0, 0, 0];

impl Board {
	pub fn eval(&self)->i32 {
		let mut score = 0;
		let mut eff = [[[0; 13]; 11]; 2];
		
		let bit_enemy = if self.turn {0x10} else {0};
		
		for n in 0 .. 7 {
			score += self.rsv[0][n] as i32 * MAT_RSV[n ^ bit_enemy];
			score += self.rsv[1][n] as i32 * MAT_RSV[n | 0x10 ^ bit_enemy];
		}
		
		for x in 0 .. 9 {
			for y in 0 .. 9 {
				let grid = self.terra[y][x];
				if grid > 0 {
					let g1 = (grid - 1) as usize;
					let g2 = g1 ^ bit_enemy;
					score += MAT_TERRA[g2];
					let e = &mut eff[g2 >> 4];
					
					for diff in LIST_MOVE_MELEE[g1] {
						e[(1 + x as i8 + diff.x) as usize][(2 + y as i8 + diff.y) as usize] += 1;
					}
					'a:for &diff in LIST_MOVE_RANGED[g1] {
						let mut p = Pos{x:x as i8, y:y as i8};
						let mut grid;
						search_ray!(diff, self, p, grid, {
							e[(1 + p.x) as usize][(2 + p.y) as usize] += 1;
							if grid > 0 {continue 'a}
						});
					}
				}
			}
		}
		
		let kpos = self.kpos[self.turn as usize];
		if kpos.x < 9 {
			for x in 0 .. 9 {
				for y in 0 .. 9 {
					let dx = (if kpos.x < x {x - kpos.x} else {kpos.x - x}) as usize;
					let dy = (if kpos.y < y {y - kpos.y} else {kpos.y - y}) as usize;
					let d = if dx < dy {dx} else {dy};
					score += EFF_FRIEND[eff[0][(x + 1) as usize][(y + 2) as usize]][d];
					score -= EFF_ENEMY[eff[1][(x + 1) as usize][(y + 2) as usize]][d];
				}
			}
			score += V_KINGRANK[kpos.invert(!self.turn).y as usize];
		}
		
		let kpos = self.kpos[!self.turn as usize];
		if kpos.x < 9 {
			for x in 0 .. 9 {
				for y in 0 .. 9 {
					let dx = (if kpos.x < x {x - kpos.x} else {kpos.x - x}) as usize;
					let dy = (if kpos.y < y {y - kpos.y} else {kpos.y - y}) as usize;
					let d = if dx < dy {dx} else {dy};
					score += EFF_ENEMY[eff[0][(x + 1) as usize][(y + 2) as usize]][d];
					score -= EFF_FRIEND[eff[1][(x + 1) as usize][(y + 2) as usize]][d];
				}
			}
			score -= V_KINGRANK[kpos.invert(self.turn).y as usize];
		}
		
		return score;
	}
}