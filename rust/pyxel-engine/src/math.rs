use std::f64::consts::PI;
use std::sync::{LazyLock, Mutex};

use noise::{NoiseFn, Perlin};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256StarStar;

use crate::pyxel::Pyxel;

static RNG: LazyLock<Mutex<Xoshiro256StarStar>> = LazyLock::new(|| {
    let seed = pyxel_platform::elapsed_time();
    Mutex::new(Xoshiro256StarStar::seed_from_u64(seed as u64))
});

static PERLIN: LazyLock<Mutex<Perlin>> = LazyLock::new(|| {
    let seed = pyxel_platform::elapsed_time();
    Mutex::new(Perlin::new(seed))
});

impl Pyxel {
    pub fn ceil(x: f64) -> i32 {
        f64::ceil(x) as i32
    }

    pub fn floor(x: f64) -> i32 {
        f64::floor(x) as i32
    }

    pub fn sgn(x: f64) -> i32 {
        if x > 0.0 {
            1
        } else if x < 0.0 {
            -1
        } else {
            0
        }
    }

    pub fn sqrt(x: f64) -> f64 {
        f64::sqrt(x)
    }

    pub fn sin(deg: f64) -> f64 {
        f64::sin(deg * PI / 180.0)
    }

    pub fn cos(deg: f64) -> f64 {
        f64::cos(deg * PI / 180.0)
    }

    pub fn atan2(y: f64, x: f64) -> f64 {
        f64::atan2(y, x) * 180.0 / PI
    }

    pub fn rseed(seed: u32) {
        let rng = Xoshiro256StarStar::seed_from_u64(seed as u64);
        *RNG.lock().unwrap() = rng;
    }

    pub fn rndi(a: i32, b: i32) -> i32 {
        let (a, b) = if a < b { (a, b) } else { (b, a) };
        RNG.lock().unwrap().random_range(a..=b)
    }

    pub fn rndf(a: f64, b: f64) -> f64 {
        let (a, b) = if a < b { (a, b) } else { (b, a) };
        RNG.lock().unwrap().random_range(a..=b)
    }

    pub fn nseed(seed: u32) {
        let perlin = Perlin::new(seed);
        *PERLIN.lock().unwrap() = perlin;
    }

    pub fn noise(x: f64, y: f64, z: f64) -> f64 {
        PERLIN.lock().unwrap().get([x, y, z])
    }
}
