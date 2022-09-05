use std::f64::consts;

use noise::{NoiseFn, Perlin, Seedable};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256StarStar;

use crate::platform::Platform;

pub struct Math {
    rng: Xoshiro256StarStar,
    perlin: Perlin,
}

unsafe_singleton!(Math);

impl Math {
    pub fn init() {
        let seed = Platform::instance().tick_count();
        let rng = Xoshiro256StarStar::seed_from_u64(seed as u64);
        let perlin = Perlin::new();
        perlin.set_seed(seed);
        Self::set_instance(Self { rng, perlin });
    }
}

pub fn ceil(x: f64) -> i32 {
    f64::ceil(x) as i32
}

pub fn floor(x: f64) -> i32 {
    f64::floor(x) as i32
}

pub fn sgn(x: f64) -> f64 {
    if x > 0.0 {
        1.0
    } else if x < 0.0 {
        -1.0
    } else {
        0.0
    }
}

pub fn sqrt(x: f64) -> f64 {
    if x == 0.0 {
        f64::MAX
    } else {
        f64::sqrt(x)
    }
}

pub fn sin(deg: f64) -> f64 {
    f64::sin(deg * consts::PI / 180.0)
}

pub fn cos(deg: f64) -> f64 {
    f64::cos(deg * consts::PI / 180.0)
}

pub fn atan2(y: f64, x: f64) -> f64 {
    f64::atan2(y, x) * 180.0 / consts::PI
}

pub fn rseed(seed: u32) {
    Math::instance().rng = Xoshiro256StarStar::seed_from_u64(seed as u64);
}

pub fn rndi(a: i32, b: i32) -> i32 {
    let (a, b) = if a < b { (a, b) } else { (b, a) };
    Math::instance().rng.gen_range(a..=b)
}

pub fn rndf(a: f64, b: f64) -> f64 {
    let (a, b) = if a < b { (a, b) } else { (b, a) };
    Math::instance().rng.gen_range(a..=b)
}

pub fn nseed(seed: u32) {
    Math::instance().perlin.set_seed(seed);
}

pub fn noise(x: f64, y: f64, z: f64) -> f64 {
    Math::instance().perlin.get([x, y, z])
}
