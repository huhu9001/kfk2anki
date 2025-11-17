use crate::board::Board;
use crate::board::Move;
use crate::moves::NaiveUIter;
use crate::transposition::{HashBase, TSPTable};

use std::sync::atomic::{AtomicBool, Ordering};

pub struct CkmResult<'a> {
	pub value:i8,
	pub mv:Option<Move>,
	tt:TSPTable<'a, CkmItem>,
	depth:u8,
}
impl<'a> CkmResult<'a> {
	pub fn new(hb:&'a HashBase)->Self {
		return Self{
			value:0,
			mv:None,
			tt:TSPTable::new(hb, 20),
			depth:0,
		};
	}
	
	const MAX_DEPTH:usize = 32;
}

struct CkmItem {
	turn:bool,
	terra:[[u8; 9]; 9],
	rsv:[u8; 7],
	value:i8,
	depth:u8,
}

impl NaiveUIter {
	fn next_check(&mut self, board:&Board)->Option<Move> {
		while let Some(mv) = self.next(board) {
			if board.will_check(mv) {
				return Some(mv);
			}
		}
		return None;
	}
}

pub fn find_checkmate(ckm:&mut CkmResult, b:&mut Board, stopper:Option<&AtomicBool>) {
	ckm.depth = 5;
	//ckm.depth = 13;
	while match stopper {
		Some(s) => !s.load(Ordering::Relaxed),
		None => true,
	} && ckm.value == 0 && ckm.depth < CkmResult::MAX_DEPTH as u8 {
		//println!("{}", ckm.depth);
		ckm.value = find_checkmate_a(ckm, b, -1, 1, ckm.depth);
		ckm.depth += 2;
	}
}

//macro_rules! pause {
//	() => {std::io::stdin().read_line(&mut String::new()).unwrap()}
//}

fn find_checkmate_a(ckm:&mut CkmResult, b:&mut Board, mut alpha:i8, beta:i8, n:u8)->i8 {
	//println!("search into");
	//println!("{b:?} {n}");
	//pause!();
	
	let key = ckm.tt.key_board(b);
	if let Some(ref item) = ckm.tt[key] {
		if item.turn == b.turn && item.terra == b.terra && item.rsv == b.rsv[0] {
			if item.value != 0 || item.depth >= n {
				//println!("tt cut with {}", item.value);
				//pause!();
				return item.value;
			}
		}
		//else {println!{"COLLISION!!!"}}
	}
	
	let mut mv_last = None;
	let v = if n <= 0 {
		if let None = b.moves_unbound().next_check(b) {-1} else {0}
	} else {
		let mut v = -1;
		let mut moves = b.moves().filter(|&it| b.will_check(it)).collect::<Vec<_>>();
		moves.sort_by_key(|&it| {
			let umv = b.do_move(it);
			let v = b.eval();
			b.undo_move(umv);
			return v;
		});
		let mut mvs = moves.into_iter();
		while let Some(mv) = mvs.next() {
		//let mut mvs = b.moves_unbound();
		//while let Some(mv) = mvs.next_check(b) {
			mv_last = Some(mv);
			let umv = b.do_move(mv);
			let value = find_checkmate_b(ckm, b, alpha, beta, n - 1);
			b.undo_move(umv);
			
			if value > v {
				v = value;
				if value > alpha {alpha = value}
			}
			if value >= beta {
				//println!("beta cut");
				break;
			}
		}
		v
	};
	
	//println!("{v} found at");
	//println!("{b:?} {n}");
	//pause!();
	
	ckm.tt[key] = Some(CkmItem{
		turn:b.turn,
		terra:b.terra.clone(),
		rsv:b.rsv[0].clone(),
		value:v,
		depth:n,
	});
	if n == ckm.depth && v == 1 {
		ckm.mv = mv_last;
	}
	
	return v;
}

fn find_checkmate_b(ckm:&mut CkmResult, b:&mut Board, alpha:i8, mut beta:i8, n:u8)->i8 {
	//println!("search into");
	//println!("{b:?} {n}");
	//pause!();
	
	let key = ckm.tt.key_board(b);
	if let Some(ref item) = ckm.tt[key] {
		if item.turn == b.turn && item.terra == b.terra && item.rsv == b.rsv[0] {
			if item.value != 0 || item.depth >= n {
				//println!("tt cut with {}", item.value);
				//pause!();
				return item.value;
			}
		}
	}
	
	let mut mv_last = None;
	let v = if n <= 0 {
		if let None = b.moves_unbound().next(b) {1} else {0}
	} else {
		let mut v = 1;
		let mut moves = b.moves().collect::<Vec<_>>();
		moves.sort_by_key(|&it| {
			let umv = b.do_move(it);
			let v = b.eval();
			b.undo_move(umv);
			return v;
		});
		let mut mvs = moves.into_iter();
		while let Some(mv) = mvs.next() {
		//let mut mvs = b.moves_unbound();
		//while let Some(mv) = mvs.next(b) {
			mv_last = Some(mv);
			let umv = b.do_move(mv);
			let value = find_checkmate_a(ckm, b, alpha, beta, n - 1);
			b.undo_move(umv);
			
			if value < v {
				v = value;
				if value < beta {beta = value}
			}
			if value <= alpha {
				//println!("beta cut");
				break;
			}
		}
		v
	};
	
	//println!("{v} found at");
	//println!("{b:?} {n}");
	//pause!();
	
	ckm.tt[key] = Some(CkmItem{
		turn:b.turn,
		terra:b.terra.clone(),
		rsv:b.rsv[0].clone(),
		value:v,
		depth:n,
	});
	if n == ckm.depth && (v == -1 || v == 0 && ckm.mv.is_none()) {
		ckm.mv = mv_last;
	}
	
	return v;
}