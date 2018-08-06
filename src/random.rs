use std::fmt;

pub struct XorShift{
   seed_: [u32; 4],
}

impl XorShift{
    pub fn next(&mut self) -> u32{
        let t : u32 = self.seed_[0] ^ (self.seed_[0] << 11);
        self.seed_[0] = self.seed_[1];
        self.seed_[1] = self.seed_[2];
        self.seed_[2] = self.seed_[3];
        self.seed_[3] = self.seed_[3] ^ (self.seed_[3] >> 19) ^ (t ^ (t >> 18));
        self.seed_[3]
    }

    pub fn next01(&mut self) -> f64{
        (self.next() / u32::max_value()) as f64
    }

    pub fn new(initial_seed: u32) -> XorShift{
        let seed = &mut [0;4];
        let mut s : u32 = initial_seed;
        let mut i : u32 = 1;
        for element in seed.iter_mut(){
            s = (1812433253_u32).wrapping_mul(s^(s>>30)) + i;
            *element = s;
            i = i + 1;
        }
        XorShift{
            seed_ : *seed
        }
    }
}

impl fmt::Display for XorShift{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "({}, {}, {}, {})",self.seed_[0], self.seed_[1], self.seed_[2], self.seed_[3])
    }
}

pub type Random = XorShift;