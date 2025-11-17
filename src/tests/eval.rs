use crate::board::Board;

#[test] fn eval_0() {
	let e1 = Board::from("
	DDP|
	RA....u.R
	.P..g....
	...P....P
	P.D.P..d.
	aK.......
	.apv....p
	p.g.ppap.
	......pk.
	rU.H.g..r
	|gppppp").eval();
			
	let e2 = Board::from("
	DDP|
	RA......R
	.P..g....
	...P....P
	P.D.P..d.
	aK....u..
	.apv....p
	p.g.ppap.
	......pk.
	rU.H.g..r
	|gppppp").eval();
	
	assert!(e1 < e2, "{e1} < {e2}");
}