pub trait RandGen<T> {
	fn next_int(&mut self)->T;
}

pub struct Mt19937 {
	state_array:[u32; Self::N],
	state_index:usize,
}
impl Mt19937 {
	const N:usize = 624;
	const M:usize = 397;
	const W:u32 = 32;
	const R:u32 = 31;
	const UMASK:u32 = 0xffffffff << Self::R;
	const LMASK:u32 = 0xffffffff >> (Self::W - Self::R);
	const A:u32 = 0x9908b0df;
	const U:u32 = 11;
	const S:u32 = 7;
	const T:u32 = 15;
	const L:u32 = 18;
	const B:u32 = 0x9d2c5680;
	const C:u32 = 0xefc60000;
	const F:u32 = 1812433253;
	
	pub fn new(mut seed:u32)->Self {
		let mut g = Self {
			state_array:[0; Self::N],
			state_index:0,
		};
		let mut i = 0;
		for s in &mut g.state_array {
			*s = seed;
			seed = Self::F.wrapping_mul(seed ^ (seed >> (Self::W - 2))) + i;
			i += 1;
		}
		return g;
	}
}
impl RandGen<u32> for Mt19937 {
	fn next_int(&mut self)->u32 {
		let arr = &mut self.state_array;
		let mut k = self.state_index;
		
		let mut j = k + 1;
		if j >= Self::N { j -= Self::N; }
		
		let mut x = (arr[k] & Self::UMASK) | (arr[j] & Self::LMASK);
		
		let mut x_a = x >> 1;
		if x_a & 0x1 != 0 { x_a ^= Self::A; }
		
		j = k + Self::M;
		if j >= Self::N { j -= Self::N; }
		
		x = arr[j] ^ x_a;
		arr[k] = x;
		
		k += 1;
		if k >= Self::N { k = 0; }
		self.state_index = k;
		
		let mut y = x ^ (x >> Self::U);
		y = y ^ ((y << Self::S) & Self::B);
		y = y ^ ((y << Self::T) & Self::C);
		
		return y ^ (y >> Self::L);
	}
}

//https://github.com/mattgallagher/CwlUtils/blob/master/Sources/ReferenceRandomGenerators/mt19937-64.c
pub struct Mt19937_64 {
	mt:[u64; Self::NN],
	mti:usize,
}
impl Mt19937_64 {
	const NN:usize = 312;
	const MATRIX_A:u64 = 0xB5026F5AA96619E9;
	const UM:u64 = 0xFFFFFFFF80000000;
	const LM:u64 = 0x7FFFFFFF;
	
	pub fn new(seed:u64)->Self {
		let mut g = Self {
			mt:[0; Self::NN],
			mti:Self::NN,
		};
		g.mt[0] = seed;
		for i in 1 .. Self::NN {
			g.mt[i] = (g.mt[i - 1] ^ (g.mt[i - 1] >> 62)).wrapping_mul(6364136223846793005) + i as u64;
		}
		return g;
	}
}
impl RandGen<u64> for Mt19937_64 {
	fn next_int(&mut self)->u64 {
		if self.mti >= Self::NN {
			const MID:usize = Mt19937_64::NN / 2;
			let state_mid = self.mt[MID];
			
			for i in 0 .. MID - 1 {
				let j = i + MID;
				let x = self.mt[i] & Self::UM | self.mt[i + 1] & Self::LM;
				self.mt[i] = self.mt[i + MID] ^ x >> 1 ^ (self.mt[i + 1] & 1) * Self::MATRIX_A;
				let y = self.mt[j] & Self::UM | self.mt[j + 1] & Self::LM;
				self.mt[j] = self.mt[j - MID] ^ y >> 1 ^ (self.mt[j + 1] & 1) * Self::MATRIX_A;
			}
			let x = self.mt[MID - 1] & Self::UM | state_mid & Self::LM;
			self.mt[MID - 1] = self.mt[Self::NN - 1] ^ x >> 1 ^ (state_mid & 1) * Self::MATRIX_A;
			let y = self.mt[Self::NN - 1] & Self::UM | self.mt[0] & Self::LM;
			self.mt[Self::NN - 1] = self.mt[MID - 1] ^ y >> 1 ^ (self.mt[0] & 1) * Self::MATRIX_A;
			
			self.mti = 0;
		}
		
		let mut result = self.mt[self.mti];
		self.mti += 1;
		
		result ^= result >> 29 & 0x5555555555555555;
		result ^= result << 17 & 0x71D67FFFEDA60000;
		result ^= result << 43 & 0xFFF7EEE000000000;
		result ^= result >> 43;
		
		return result;
	}
}

#[allow(non_camel_case_types)] pub struct Mt19937_S {
	#[cfg(target_pointer_width = "32")] g:Mt19937,
	#[cfg(target_pointer_width = "64")] g:Mt19937_64,
}
impl Mt19937_S {
	pub fn new(seed:usize)->Self {
		return Self {
			#[cfg(target_pointer_width = "32")] g:Mt19937::new(seed as u32),
			#[cfg(target_pointer_width = "64")] g:Mt19937_64::new(seed as u64),
		};
	}
}
impl RandGen<usize> for Mt19937_S {
	fn next_int(&mut self)->usize {
		return self.g.next_int() as usize;
	}
}

pub fn seed_default()->u128 {
	return std::time::UNIX_EPOCH.elapsed().unwrap().as_millis();
}