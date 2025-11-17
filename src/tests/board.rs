use crate::board::Board;

#[test] fn undo_move_0() {
	let b1 = Board::from("|
	u......GK
	......z..
	......a.g
	.........
	.........
	.........
	.........
	.........
	.........
	|p");
	let mut b2 = b1.clone();
	for mv in b1.moves() {
		assert!(b1.legal(mv), "{mv:?}");
		let umv = b2.do_move(mv);
		assert_eq!(b2.valid(), 0);
		b2.undo_move(umv);
		assert_eq!(b2.valid(), 0);
		assert_eq!(b1, b2);
	}
}