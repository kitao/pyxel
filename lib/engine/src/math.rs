use std::f64::consts;

use noise::{NoiseFn, Perlin, Seedable};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256StarStar;

use crate::Platform;
use crate::Pyxel;

pub struct Math {
    rng: Xoshiro256StarStar,
    perlin: Perlin,
}

impl Math {
    pub fn new<T: Platform>(platform: &mut T) -> Self {
        let seed = platform.tick_count();
        let rng = Xoshiro256StarStar::seed_from_u64(seed as u64);
        let perlin = Perlin::new();
        perlin.set_seed(seed);
        Self { rng, perlin }
    }
}

impl Pyxel {
    pub fn sgn(&self, x: f64) -> f64 {
        if x > 0.0 {
            1.0
        } else if x < 0.0 {
            -1.0
        } else {
            0.0
        }
    }

    pub fn sqrt(&self, x: f64) -> f64 {
        if x == 0.0 {
            f64::MAX
        } else {
            f64::sqrt(x)
        }
    }

    pub fn sin(&self, deg: f64) -> f64 {
        f64::sin(deg * consts::PI / 180.0)
    }

    pub fn cos(&self, deg: f64) -> f64 {
        f64::cos(deg * consts::PI / 180.0)
    }

    pub fn atan2(&self, y: f64, x: f64) -> f64 {
        f64::atan2(y, x) * 180.0 / consts::PI
    }

    pub fn srand(&mut self, seed: u32) {
        self.math.rng = Xoshiro256StarStar::seed_from_u64(seed as u64);
    }

    pub fn rndi(&mut self, a: i32, b: i32) -> i32 {
        let (a, b) = if a < b { (a, b) } else { (b, a) };
        self.math.rng.gen_range(a..=b)
    }

    pub fn rndf(&mut self, a: f64, b: f64) -> f64 {
        let (a, b) = if a < b { (a, b) } else { (b, a) };
        self.math.rng.gen_range(a..=b)
    }

    pub fn nseed(&mut self, seed: u32) {
        self.math.perlin.set_seed(seed);
    }

    pub fn noise(&self, x: f64, y: f64, z: f64) -> f64 {
        self.math.perlin.get([x, y, z])
    }
}
