use crate::board::*;
use crate::checkmate::*;
use crate::transposition::*;
use crate::rand::*;

use std::sync::atomic::{AtomicBool, Ordering};
static stopper:AtomicBool = AtomicBool::new(false);

fn ckm(s:&str, found:bool)->Move {
	let seed = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis() as usize;
	let hb = HashBase::new(&mut Mt19937_S::new(seed));
	let mut b = Board::from(s);
	let mut ckm = CkmResult::new(&hb);
	std::thread::spawn(|| {
		std::thread::sleep(std::time::Duration::from_secs(5));
		stopper.store(true, Ordering::Relaxed);
	});
	find_checkmate(&mut ckm, &mut b, Some(&stopper));
	if found {assert_eq!(ckm.value, 1)}
	else {assert_ne!(ckm.value, 1)}
	return ckm.mv.expect("should checkmate");
}

#[test]fn ckm_test_0() {
	let mv = ckm(
	"APPPPPP|
	R.h...c.R
	..D.dG...
	P..P...P.
	..P.Pg..K
	...A.P..P
	..pkp.a..
	p.g..pd.p
	..g......
	r...c..ar
	|hd", true);
	assert!(mv.to.x == 1 && mv.to.y == 4, "({},{})-({},{})", mv.from.x, mv.from.y, mv.to.x, mv.to.y);
}

#[test]fn ckm_test_1() {
	ckm("DP|
	RA....u.R
	.P..g....
	..DP....P
	P.P.P..d.
	aK.d.....
	.apv....p
	p.g.ppap.
	......pk.
	rU.H.g..r
	|GPPPP", true);
}