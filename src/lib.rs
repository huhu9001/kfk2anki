#![allow(unused_braces)]
#![allow(non_upper_case_globals)]

pub mod board;
pub mod moves;
pub mod eval;
pub mod rand;
pub mod bdgen;
pub mod checkmate;
pub mod transposition;

#[cfg(all(test, debug_assertions))] mod tests {
	mod board;
	mod moves;
	mod eval;
	mod bdgen;
}
#[cfg(all(test, not(debug_assertions)))] mod benches {
	mod checkmate;
}