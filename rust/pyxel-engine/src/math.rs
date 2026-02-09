use std::f32::consts::PI;
use std::sync::{LazyLock, Mutex};

use noise::{NoiseFn, Perlin};
use rand::rngs::OsRng;
use rand::{Rng, SeedableRng, TryRngCore};
use rand_xoshiro::Xoshiro256StarStar;

use crate::pyxel::Pyxel;

static RNG: LazyLock<Mutex<Xoshiro256StarStar>> =
    LazyLock::new(|| Mutex::new(Xoshiro256StarStar::from_os_rng()));

static PERLIN: LazyLock<Mutex<Perlin>> =
    LazyLock::new(|| Mutex::new(Perlin::new(OsRng.try_next_u32().unwrap())));

impl Pyxel {
    pub fn ceil(x: f32) -> i32 {
        f32::ceil(x) as i32
    }

    pub fn floor(x: f32) -> i32 {
        f32::floor(x) as i32
    }

    pub fn clamp(x: f32, lower: f32, upper: f32) -> f32 {
        x.clamp(lower, upper)
    }

    pub fn sgn(x: f32) -> i32 {
        if x > 0.0 {
            1
        } else if x < 0.0 {
            -1
        } else {
            0
        }
    }

    pub fn sqrt(x: f32) -> f32 {
        f32::sqrt(x)
    }

    pub fn sin(deg: f32) -> f32 {
        f32::sin(deg * PI / 180.0)
    }

    pub fn cos(deg: f32) -> f32 {
        f32::cos(deg * PI / 180.0)
    }

    pub fn atan2(y: f32, x: f32) -> f32 {
        f32::atan2(y, x) * 180.0 / PI
    }

    pub fn rseed(seed: u32) {
        let rng = Xoshiro256StarStar::seed_from_u64(seed as u64);
        *RNG.lock().unwrap() = rng;
    }

    pub fn rndi(a: i32, b: i32) -> i32 {
        let (a, b) = if a < b { (a, b) } else { (b, a) };
        RNG.lock().unwrap().random_range(a..=b)
    }

    pub fn rndf(a: f32, b: f32) -> f32 {
        let (a, b) = if a < b { (a, b) } else { (b, a) };
        RNG.lock().unwrap().random_range(a..=b)
    }

    pub fn nseed(seed: u32) {
        let perlin = Perlin::new(seed);
        *PERLIN.lock().unwrap() = perlin;
    }

    pub fn noise(x: f32, y: f32, z: f32) -> f32 {
        PERLIN.lock().unwrap().get([x as f64, y as f64, z as f64]) as f32
    }
}
