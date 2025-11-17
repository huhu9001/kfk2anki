use crate::board::Pos;
use crate::board::pid;

pub const fn can_move()->[[[u8; 17]; 17]; 0x20] {
	let mut r = [[[0; 17]; 17]; 0x20];
		
	//BLACK
	let rk = &mut r[pid::GUARD];
	rk[8][7] = 1;
	rk[8][9] = 1;
	rk[7][8] = 1;
	rk[9][8] = 1;
	rk[7][7] = 1;
	rk[9][7] = 1;
	
	let rk = &mut r[pid::DOG];
	rk[8][7] = 1;
	rk[7][7] = 1;
	rk[9][7] = 1;
	rk[7][9] = 1;
	rk[9][9] = 1;
	r[pid::DOG | 8] = r[pid::GUARD];
	
	let rk = &mut r[pid::ARCHER];
	rk[7][6] = 1;
	rk[9][6] = 1;
	r[pid::ARCHER | 8] = r[pid::GUARD];
	
	let rk = &mut r[pid::RAM];
	rk[8][7] = 1;
	let mut n = 10;
	while n <= 16 {
		rk[8][16 - n] = 2;
		n += 1;
	}
	r[pid::RAM | 8] = r[pid::GUARD];
	
	let rk = &mut r[pid::PAWN];
	rk[8][7] = 1;
	r[pid::PAWN | 8] = r[pid::GUARD];
	
	let rk = &mut r[pid::CHARIOT];
	rk[8][7] = 1;
	rk[8][9] = 1;
	rk[7][8] = 1;
	rk[9][8] = 1;
	let mut n = 0;
	while n <= 6 {
		rk[8][16 - n] = 2;
		rk[8][n] = 2;
		rk[16 - n][8] = 2;
		rk[n][8] = 2;
		n += 1;
	}
	r[pid::CHARIOT | 8] = *rk;
	let rkp = &mut r[pid::CHARIOT | 8];
	rkp[7][7] = 1;
	rkp[9][7] = 1;
	rkp[7][9] = 1;
	rkp[9][9] = 1;
	
	let rk = &mut r[pid::HORSE];
	rk[7][7] = 1;
	rk[9][7] = 1;
	rk[7][9] = 1;
	rk[9][9] = 1;
	let mut n = 0;
	while n <= 6 {
		rk[16 - n][16 - n] = 2;
		rk[n][n] = 2;
		rk[16 - n][n] = 2;
		rk[n][16 - n] = 2;
		n += 1;
	}
	r[pid::HORSE | 8] = *rk;
	let rkp = &mut r[pid::HORSE | 8];
	rkp[8][7] = 1;
	rkp[8][9] = 1;
	rkp[7][8] = 1;
	rkp[9][8] = 1;
	
	let rk = &mut r[pid::KING];
	rk[8][9] = 1;
	rk[8][7] = 1;
	rk[9][8] = 1;
	rk[7][8] = 1;
	rk[9][9] = 1;
	rk[7][9] = 1;
	rk[9][7] = 1;
	rk[7][7] = 1;
	
	//WHITE
	let rk = &mut r[pid::GUARD | 0x10];
	rk[8][9] = 1;
	rk[8][7] = 1;
	rk[9][8] = 1;
	rk[7][8] = 1;
	rk[9][9] = 1;
	rk[7][9] = 1;
	
	let rk = &mut r[pid::DOG | 0x10];
	rk[8][9] = 1;
	rk[9][9] = 1;
	rk[7][9] = 1;
	rk[9][7] = 1;
	rk[7][7] = 1;
	r[pid::DOG | 0x18] = r[pid::GUARD | 0x10];
	
	let rk = &mut r[pid::ARCHER | 0x10];
	rk[9][10] = 1;
	rk[7][10] = 1;
	r[pid::ARCHER | 0x18] = r[pid::GUARD | 0x10];
	
	let rk = &mut r[pid::RAM | 0x10];
	rk[8][9] = 1;
	let mut n = 10;
	while n <= 16 {
		rk[8][n] = 2;
		n += 1;
	}
	r[pid::RAM | 0x18] = r[pid::GUARD | 0x10];
	
	let rk = &mut r[pid::PAWN | 0x10];
	rk[8][9] = 1;
	r[pid::PAWN | 0x18] = r[pid::GUARD | 0x10];
	
	let rk = &mut r[pid::CHARIOT | 0x10];
	rk[8][9] = 1;
	rk[8][7] = 1;
	rk[9][8] = 1;
	rk[7][8] = 1;
	let mut n = 0;
	while n <= 6 {
		rk[8][n] = 2;
		rk[8][16 - n] = 2;
		rk[n][8] = 2;
		rk[16 - n][8] = 2;
		n += 1;
	}
	r[pid::CHARIOT | 0x18] = *rk;
	let rkp = &mut r[pid::CHARIOT | 0x18];
	rkp[9][9] = 1;
	rkp[7][9] = 1;
	rkp[9][7] = 1;
	rkp[7][7] = 1;
	
	let rk = &mut r[pid::HORSE | 0x10];
	rk[9][9] = 1;
	rk[7][9] = 1;
	rk[9][7] = 1;
	rk[7][7] = 1;
	let mut n = 0;
	while n <= 6 {
		rk[n][n] = 2;
		rk[16 - n][16 - n] = 2;
		rk[n][16 - n] = 2;
		rk[16 - n][n] = 2;
		n += 1;
	}
	r[pid::HORSE | 0x18] = *rk;
	let rkp = &mut r[pid::HORSE | 0x18];
	rkp[8][9] = 1;
	rkp[8][7] = 1;
	rkp[9][8] = 1;
	rkp[7][8] = 1;
	
	let rk = &mut r[pid::KING | 0x10];
	rk[8][9] = 1;
	rk[8][7] = 1;
	rk[9][8] = 1;
	rk[7][8] = 1;
	rk[9][9] = 1;
	rk[7][9] = 1;
	rk[9][7] = 1;
	rk[7][7] = 1;
	
	return r;
}

pub const fn list_move_melee()->[&'static[Pos]; 0x20] {
	let mut r = [[].as_slice(); 0x20];
	
	//BLACK
	r[pid::GUARD] = &[
		Pos{x:0, y:-1},
		Pos{x:0, y:1},
		Pos{x:-1, y:0},
		Pos{x:1, y:0},
		Pos{x:-1, y:-1},
		Pos{x:1, y:-1},
	];
	
	r[pid::DOG] = &[
		Pos{x:0, y:-1},
		Pos{x:-1, y:-1},
		Pos{x:1, y:1},
		Pos{x:-1, y:1},
		Pos{x:1, y:-1},
	];
	r[pid::DOG | 8] = r[pid::GUARD];
	
	r[pid::ARCHER] = &[
		Pos{x:-1, y:-2},
		Pos{x:1, y:-2},
	];
	r[pid::ARCHER | 8] = r[pid::GUARD];
	
	r[pid::RAM | 8] = r[pid::GUARD];
	
	r[pid::PAWN] = &[
		Pos{x:0, y:-1},
	];
	r[pid::PAWN | 8] = r[pid::GUARD];
	
	r[pid::CHARIOT | 8] = &[
		Pos{x:-1, y:-1},
		Pos{x:1, y:1},
		Pos{x:-1, y:1},
		Pos{x:1, y:-1},
	];
	r[pid::HORSE | 8] = &[
		Pos{x:0, y:-1},
		Pos{x:0, y:1},
		Pos{x:-1, y:0},
		Pos{x:1, y:0},
	];
	
	r[pid::KING] = &[
		Pos{x:0, y:-1},
		Pos{x:0, y:1},
		Pos{x:-1, y:0},
		Pos{x:1, y:0},
		Pos{x:-1, y:-1},
		Pos{x:1, y:1},
		Pos{x:-1, y:1},
		Pos{x:1, y:-1},
	];
	
	//WHITE
	r[pid::GUARD | 0x10] = &[
		Pos{x:0, y:1},
		Pos{x:0, y:-1},
		Pos{x:1, y:0},
		Pos{x:-1, y:0},
		Pos{x:1, y:1},
		Pos{x:-1, y:1},
	];
	
	r[pid::DOG | 0x10] = &[
		Pos{x:0, y:1},
		Pos{x:1, y:1},
		Pos{x:-1, y:-1},
		Pos{x:1, y:-1},
		Pos{x:-1, y:1},
	];
	r[pid::DOG | 0x18] = r[pid::GUARD | 0x10];
	
	r[pid::ARCHER | 0x10] = &[
		Pos{x:1, y:2},
		Pos{x:-1, y:2},
	];
	r[pid::ARCHER | 0x18] = r[pid::GUARD | 0x10];
	
	r[pid::RAM | 0x18] = r[pid::GUARD | 0x10];
	
	r[pid::PAWN | 0x10] = &[
		Pos{x:0, y:1},
	];
	r[pid::PAWN | 0x18] = r[pid::GUARD | 0x10];
	
	r[pid::CHARIOT | 0x18] = &[
		Pos{x:1, y:1},
		Pos{x:-1, y:-1},
		Pos{x:1, y:-1},
		Pos{x:-1, y:1},
	];
	r[pid::HORSE | 0x18] = &[
		Pos{x:0, y:1},
		Pos{x:0, y:-1},
		Pos{x:1, y:0},
		Pos{x:-1, y:0},
	];
	
	r[pid::KING | 0x10] = &[
		Pos{x:0, y:1},
		Pos{x:0, y:-1},
		Pos{x:1, y:0},
		Pos{x:-1, y:0},
		Pos{x:1, y:1},
		Pos{x:-1, y:-1},
		Pos{x:1, y:-1},
		Pos{x:-1, y:1},
	];
	
	return r;
}

pub const fn list_move_ranged()->[&'static[Pos]; 0x20] {
	let mut r = [[].as_slice(); 0x20];
	
	//BLACK
	r[pid::RAM] = &[
		Pos{x:0, y:-1},
	];
	r[pid::CHARIOT] = &[
		Pos{x:0, y:-1},
		Pos{x:0, y:1},
		Pos{x:-1, y:0},
		Pos{x:1, y:0},
	];
	r[pid::CHARIOT | 8] = r[pid::CHARIOT];
	r[pid::HORSE] = &[
		Pos{x:-1, y:-1},
		Pos{x:1, y:1},
		Pos{x:-1, y:1},
		Pos{x:1, y:-1},
	];
	r[pid::HORSE | 8] = r[pid::HORSE];
	
	//WHITE
	r[pid::RAM | 0x10] = &[
		Pos{x:0, y:1},
	];
	r[pid::CHARIOT | 0x10] = &[
		Pos{x:0, y:1},
		Pos{x:0, y:-1},
		Pos{x:1, y:0},
		Pos{x:-1, y:0},
	];
	r[pid::CHARIOT | 0x18] = r[pid::CHARIOT | 0x10];
	r[pid::HORSE | 0x10] = &[
		Pos{x:1, y:1},
		Pos{x:-1, y:-1},
		Pos{x:1, y:-1},
		Pos{x:-1, y:1},
	];
	r[pid::HORSE | 0x18] = r[pid::HORSE | 0x10];
	
	return r;
}

pub const fn can_prmt()->[[u8; 9]; 0x20] {
	let mut r = [[0; 9]; 0x20];
	
	let mut y = 0;
	while y < 3 {
		r[pid::DOG][y] = 1;
		r[pid::CHARIOT][y] = 1;
		r[pid::HORSE][y] = 1;
		
		r[pid::DOG | 0x10][8 - y] = 1;
		r[pid::CHARIOT | 0x10][8 - y] = 1;
		r[pid::HORSE | 0x10][8 - y] = 1;
		
		y += 1;
	}
	
	r[pid::ARCHER][0] = 3;
	r[pid::ARCHER][1] = 3;
	r[pid::ARCHER][2] = 1;
	r[pid::RAM][0] = 3;
	r[pid::RAM][1] = 1;
	r[pid::RAM][2] = 1;
	r[pid::PAWN][0] = 3;
	r[pid::PAWN][1] = 1;
	r[pid::PAWN][2] = 1;
	
	r[pid::ARCHER | 0x10][8] = 3;
	r[pid::ARCHER | 0x10][7] = 3;
	r[pid::ARCHER | 0x10][6] = 1;
	r[pid::RAM | 0x10][8] = 3;
	r[pid::RAM | 0x10][7] = 1;
	r[pid::RAM | 0x10][6] = 1;
	r[pid::PAWN | 0x10][8] = 3;
	r[pid::PAWN | 0x10][7] = 1;
	r[pid::PAWN | 0x10][6] = 1;
	
	return r;
}

pub const fn attackers_ranged()->[[[[bool; 0x20]; 3]; 3]; 2] {
	let mut ret = [[[[false; 0x20]; 3]; 3]; 2];
	
	//BLACK
	//(0, 1)
	ret[0][1][2][pid::RAM] = true;
	ret[0][1][2][pid::CHARIOT] = true;
	ret[0][1][2][pid::CHARIOT | 8] = true;
	//(0, -1)
	ret[0][1][0][pid::CHARIOT] = true;
	ret[0][1][0][pid::CHARIOT | 8] = true;
	//(1, 0)
	ret[0][2][1][pid::CHARIOT] = true;
	ret[0][2][1][pid::CHARIOT | 8] = true;
	//(-1, 0)
	ret[0][0][1][pid::CHARIOT] = true;
	ret[0][0][1][pid::CHARIOT | 8] = true;
	//(1, 1)
	ret[0][2][2][pid::HORSE] = true;
	ret[0][2][2][pid::HORSE | 8] = true;
	//(-1, -1)
	ret[0][0][0][pid::HORSE] = true;
	ret[0][0][0][pid::HORSE | 8] = true;
	//(1, -1)
	ret[0][2][0][pid::HORSE] = true;
	ret[0][2][0][pid::HORSE | 8] = true;
	//(-1, 1)
	ret[0][0][2][pid::HORSE] = true;
	ret[0][0][2][pid::HORSE | 8] = true;
	
	//WHITE
	//(0, 1)
	ret[1][1][0][pid::RAM | 0x10] = true;
	ret[1][1][0][pid::CHARIOT | 0x10] = true;
	ret[1][1][0][pid::CHARIOT | 0x18] = true;
	//(0, -1)
	ret[1][1][2][pid::CHARIOT | 0x10] = true;
	ret[1][1][2][pid::CHARIOT | 0x18] = true;
	//(1, 0)
	ret[1][0][1][pid::CHARIOT | 0x10] = true;
	ret[1][0][1][pid::CHARIOT | 0x18] = true;
	//(-1, 0)
	ret[1][2][1][pid::CHARIOT | 0x10] = true;
	ret[1][2][1][pid::CHARIOT | 0x18] = true;
	//(1, 1)
	ret[1][0][0][pid::HORSE | 0x10] = true;
	ret[1][0][0][pid::HORSE | 0x18] = true;
	//(-1, -1)
	ret[1][2][2][pid::HORSE | 0x10] = true;
	ret[1][2][2][pid::HORSE | 0x18] = true;
	//(1, -1)
	ret[1][0][2][pid::HORSE | 0x10] = true;
	ret[1][0][2][pid::HORSE | 0x18] = true;
	//(-1, 1)
	ret[1][2][0][pid::HORSE | 0x10] = true;
	ret[1][2][0][pid::HORSE | 0x18] = true;
	
	return ret;
}

pub const fn attackers_melee()->[[[[bool; 0x20]; 3]; 3]; 2] {
	let mut ret = attackers_ranged();
	
	//BLACK
	//(0, 1)
	ret[0][1][2][pid::GUARD] = true;
	ret[0][1][2][pid::DOG] = true;
	ret[0][1][2][pid::DOG | 8] = true;
	ret[0][1][2][pid::ARCHER | 8] = true;
	ret[0][1][2][pid::RAM | 8] = true;
	ret[0][1][2][pid::PAWN] = true;
	ret[0][1][2][pid::PAWN | 8] = true;
	ret[0][1][2][pid::HORSE | 8] = true;
	ret[0][1][2][pid::KING] = true;
	//(0, -1)
	ret[0][1][0][pid::GUARD] = true;
	ret[0][1][0][pid::DOG | 8] = true;
	ret[0][1][0][pid::ARCHER | 8] = true;
	ret[0][1][0][pid::RAM | 8] = true;
	ret[0][1][0][pid::PAWN | 8] = true;
	ret[0][1][0][pid::HORSE | 8] = true;
	ret[0][1][0][pid::KING] = true;
	//(1, 0)
	ret[0][2][1][pid::GUARD] = true;
	ret[0][2][1][pid::DOG | 8] = true;
	ret[0][2][1][pid::ARCHER | 8] = true;
	ret[0][2][1][pid::RAM | 8] = true;
	ret[0][2][1][pid::PAWN | 8] = true;
	ret[0][2][1][pid::HORSE | 8] = true;
	ret[0][2][1][pid::KING] = true;
	//(-1, 0)
	ret[0][0][1][pid::GUARD] = true;
	ret[0][0][1][pid::DOG | 8] = true;
	ret[0][0][1][pid::ARCHER | 8] = true;
	ret[0][0][1][pid::RAM | 8] = true;
	ret[0][0][1][pid::PAWN | 8] = true;
	ret[0][0][1][pid::HORSE | 8] = true;
	ret[0][0][1][pid::KING] = true;
	//(1, 1)
	ret[0][2][2][pid::GUARD] = true;
	ret[0][2][2][pid::DOG] = true;
	ret[0][2][2][pid::DOG | 8] = true;
	ret[0][2][2][pid::ARCHER | 8] = true;
	ret[0][2][2][pid::RAM | 8] = true;
	ret[0][2][2][pid::PAWN | 8] = true;
	ret[0][2][2][pid::CHARIOT | 8] = true;
	ret[0][2][2][pid::KING] = true;
	//(-1, -1)
	ret[0][0][0][pid::DOG] = true;
	ret[0][0][0][pid::CHARIOT | 8] = true;
	ret[0][0][0][pid::KING] = true;
	//(1, -1)
	ret[0][2][0][pid::DOG] = true;
	ret[0][2][0][pid::CHARIOT | 8] = true;
	ret[0][2][0][pid::KING] = true;
	//(-1, 1)
	ret[0][0][2][pid::GUARD] = true;
	ret[0][0][2][pid::DOG] = true;
	ret[0][0][2][pid::DOG | 8] = true;
	ret[0][0][2][pid::ARCHER | 8] = true;
	ret[0][0][2][pid::RAM | 8] = true;
	ret[0][0][2][pid::PAWN | 8] = true;
	ret[0][0][2][pid::CHARIOT | 8] = true;
	ret[0][0][2][pid::KING] = true;
	
	//WHITE
	//(0, 1)
	ret[1][1][0][pid::GUARD | 0x10] = true;
	ret[1][1][0][pid::DOG | 0x10] = true;
	ret[1][1][0][pid::DOG | 0x18] = true;
	ret[1][1][0][pid::ARCHER | 0x18] = true;
	ret[1][1][0][pid::RAM | 0x18] = true;
	ret[1][1][0][pid::PAWN | 0x10] = true;
	ret[1][1][0][pid::PAWN | 0x18] = true;
	ret[1][1][0][pid::HORSE | 0x18] = true;
	ret[1][1][0][pid::KING | 0x10] = true;
	//(0, -1)
	ret[1][1][2][pid::GUARD | 0x10] = true;
	ret[1][1][2][pid::DOG | 0x18] = true;
	ret[1][1][2][pid::ARCHER | 0x18] = true;
	ret[1][1][2][pid::RAM | 0x18] = true;
	ret[1][1][2][pid::PAWN | 0x18] = true;
	ret[1][1][2][pid::HORSE | 0x18] = true;
	ret[1][1][2][pid::KING | 0x10] = true;
	//(1, 0)
	ret[1][0][1][pid::GUARD | 0x10] = true;
	ret[1][0][1][pid::DOG | 0x18] = true;
	ret[1][0][1][pid::ARCHER | 0x18] = true;
	ret[1][0][1][pid::RAM | 0x18] = true;
	ret[1][0][1][pid::PAWN | 0x18] = true;
	ret[1][0][1][pid::HORSE | 0x18] = true;
	ret[1][0][1][pid::KING | 0x10] = true;
	//(-1, 0)
	ret[1][2][1][pid::GUARD | 0x10] = true;
	ret[1][2][1][pid::DOG | 0x18] = true;
	ret[1][2][1][pid::ARCHER | 0x18] = true;
	ret[1][2][1][pid::RAM | 0x18] = true;
	ret[1][2][1][pid::PAWN | 0x18] = true;
	ret[1][2][1][pid::HORSE | 0x18] = true;
	ret[1][2][1][pid::KING | 0x10] = true;
	//(1, 1)
	ret[1][0][0][pid::GUARD | 0x10] = true;
	ret[1][0][0][pid::DOG | 0x10] = true;
	ret[1][0][0][pid::DOG | 0x18] = true;
	ret[1][0][0][pid::ARCHER | 0x18] = true;
	ret[1][0][0][pid::RAM | 0x18] = true;
	ret[1][0][0][pid::PAWN | 0x18] = true;
	ret[1][0][0][pid::CHARIOT | 0x18] = true;
	ret[1][0][0][pid::KING | 0x10] = true;
	//(-1, -1)
	ret[1][2][2][pid::DOG | 0x10] = true;
	ret[1][2][2][pid::CHARIOT | 0x18] = true;
	ret[1][2][2][pid::KING | 0x10] = true;
	//(1, -1)
	ret[1][0][2][pid::DOG | 0x10] = true;
	ret[1][0][2][pid::CHARIOT | 0x18] = true;
	ret[1][0][2][pid::KING | 0x10] = true;
	//(-1, 1)
	ret[1][2][0][pid::GUARD | 0x10] = true;
	ret[1][2][0][pid::DOG | 0x10] = true;
	ret[1][2][0][pid::DOG | 0x18] = true;
	ret[1][2][0][pid::ARCHER | 0x18] = true;
	ret[1][2][0][pid::RAM | 0x18] = true;
	ret[1][2][0][pid::PAWN | 0x18] = true;
	ret[1][2][0][pid::CHARIOT | 0x18] = true;
	ret[1][2][0][pid::KING | 0x10] = true;
	
	return ret;
}