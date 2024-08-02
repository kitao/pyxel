use std::f64::consts::PI;

use noise::{NoiseFn, Perlin};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256StarStar;

use crate::pyxel::Pyxel;

pub struct Math {
    rng: Xoshiro256StarStar,
    perlin: Perlin,
}

impl Math {
    pub fn new() -> Self {
        let seed = pyxel_platform::elapsed_time();
        let rng = Xoshiro256StarStar::seed_from_u64(seed as u64);
        let perlin = Perlin::new(seed);
        Self { rng, perlin }
    }
}

impl Pyxel {
    pub fn ceil(&self, x: f64) -> i32 {
        f64::ceil(x) as i32
    }

    pub fn floor(&self, x: f64) -> i32 {
        f64::floor(x) as i32
    }

    pub fn sgn(&self, x: f64) -> i32 {
        if x > 0.0 {
            1
        } else if x < 0.0 {
            -1
        } else {
            0
        }
    }

    pub fn sqrt(&self, x: f64) -> f64 {
        f64::sqrt(x)
    }

    pub fn sin(&self, deg: f64) -> f64 {
        f64::sin(deg * PI / 180.0)
    }

    pub fn cos(&self, deg: f64) -> f64 {
        f64::cos(deg * PI / 180.0)
    }

    pub fn atan2(&self, y: f64, x: f64) -> f64 {
        f64::atan2(y, x) * 180.0 / PI
    }

    pub fn rseed(&mut self, seed: u32) {
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
        self.math.perlin = Perlin::new(seed);
    }

    pub fn noise(&mut self, x: f64, y: f64, z: f64) -> f64 {
        self.math.perlin.get([x, y, z])
    }
}
