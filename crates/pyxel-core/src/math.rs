use std::f32::consts::PI;
use std::sync::{LazyLock, Mutex};

use noise::{NoiseFn, Perlin};
use rand::{RngExt, SeedableRng};
use rand_xoshiro::Xoshiro256StarStar;

use crate::pyxel::Pyxel;

static RNG: LazyLock<Mutex<Xoshiro256StarStar>> = LazyLock::new(|| {
    let mut rng = rand::rng();
    Mutex::new(Xoshiro256StarStar::from_rng(&mut rng))
});

static PERLIN: LazyLock<Mutex<Perlin>> =
    LazyLock::new(|| Mutex::new(Perlin::new(rand::rng().random())));

impl Pyxel {
    pub fn ceil(x: f32) -> i32 {
        f32::ceil(x) as i32
    }

    pub fn floor(x: f32) -> i32 {
        f32::floor(x) as i32
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

    pub fn random_seed(seed: u32) {
        let rng = Xoshiro256StarStar::seed_from_u64(seed as u64);
        *RNG.lock().unwrap() = rng;
    }

    pub fn random_int(min: i32, max: i32) -> i32 {
        let (min, max) = if min < max { (min, max) } else { (max, min) };
        RNG.lock().unwrap().random_range(min..=max)
    }

    pub fn random_float(min: f32, max: f32) -> f32 {
        let (min, max) = if min < max { (min, max) } else { (max, min) };
        RNG.lock().unwrap().random_range(min..=max)
    }

    pub fn noise_seed(seed: u32) {
        let perlin = Perlin::new(seed);
        *PERLIN.lock().unwrap() = perlin;
    }

    pub fn noise(x: f32, y: f32, z: f32) -> f32 {
        PERLIN.lock().unwrap().get([x as f64, y as f64, z as f64]) as f32
    }
}
