use crate::board::Board;
use crate::rand::Mt19937_S;

#[test] fn good_gen() {
	for n in 0 .. 1000 {
		let b = Board::random_monoking(&mut Mt19937_S::new(n));
		assert_eq!(b.valid(), 0, "{b:?}");
	}
}