use crate::board::Board;
use crate::board::Move;
use crate::rand::RandGen;

pub struct HashBase {
	turn:[usize; 2],
	terra:[[[usize; 33]; 9]; 9],
	rsv:[[usize; 19]; 7],
}
impl HashBase {
	pub fn new<RGen:RandGen<usize>>(rgen:&mut RGen)->Self {
		let mut s = Self{
			turn:[0, rgen.next_int()],
			terra:[[[0; 33]; 9]; 9],
			rsv:[[0; 19]; 7],
		};
		for g in &mut s.terra {
			for g in g {
				for i in 1 .. 33 {
					g[i] = rgen.next_int();
				}
			}
		}
		for r in &mut s.rsv {
			for i in 1 .. 19 {
				r[i] = rgen.next_int();
			}
		}
		return s;
	}
}

pub struct TSPItem {
	pub key:usize,
	pub mv:Move,
	pub depth:u8,
	pub value:i32,
}

pub struct TSPTable<'a, Item> {
	data:Box<[Option<Item>]>,
	base:&'a HashBase,
	mask:usize,
}
impl<'a, Item> TSPTable<'a, Item> {
	pub fn new(base:&'a HashBase, size:usize)->Self {
		if (1 << size) * size_of::<Option<Item>>() > 1 << 30 { panic!("gt 1GB"); }
		let mut data = Box::<[Option<Item>]>::new_uninit_slice(1 << size);
		for d in &mut data { d.write(None); }
		return Self{
			data:unsafe {data.assume_init()},
			base,
			mask:(1 << size) - 1,
		};
	}
	
	pub fn key_terra(&self, b:&Board)->usize {
		let base = self.base;
		let mut i = 0;
		for x in 0 .. 9 {
			for y in 0 .. 9 {
				i ^= base.terra[x][y][b.terra[y][x] as usize];
			}
		}
		return i;
	}
	
	pub fn key_board(&self, b:&Board)->usize {
		let base = self.base;
		let mut i = self.key_terra(b);
		i ^= base.turn[b.turn as usize];
		for n in 0 .. 7 {
			i ^= base.rsv[n][b.rsv[0][n] as usize];
		}
		return i;
	}
}
impl<Item> std::ops::Index<usize> for TSPTable<'_, Item> {
	type Output = Option<Item>;
	fn index(&self, i:usize)->&Self::Output {
		return unsafe{self.data.get_unchecked(i & self.mask)};
	}
}
impl<Item> std::ops::IndexMut<usize> for TSPTable<'_, Item> {
	fn index_mut(&mut self, i:usize)->&mut Self::Output {
		return unsafe{self.data.get_unchecked_mut(i & self.mask)};
	}
}