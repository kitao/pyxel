use std::f32::consts::PI;
use std::sync::{LazyLock, Mutex};

use noise::{NoiseFn, Perlin};
use rand::{RngExt, SeedableRng};
use rand_xoshiro::Xoshiro256StarStar;

use crate::pyxel::Pyxel;

const DEG_TO_RAD: f32 = PI / 180.0;
const RAD_TO_DEG: f32 = 180.0 / PI;

static RNG: LazyLock<Mutex<Xoshiro256StarStar>> = LazyLock::new(|| {
    let mut rng = rand::rng();
    Mutex::new(Xoshiro256StarStar::from_rng(&mut rng))
});

static PERLIN: LazyLock<Mutex<Perlin>> =
    LazyLock::new(|| Mutex::new(Perlin::new(rand::rng().random())));

impl Pyxel {
    pub fn ceil(x: f32) -> i32 {
        x.ceil() as i32
    }

    pub fn floor(x: f32) -> i32 {
        x.floor() as i32
    }

    pub fn sqrt(x: f32) -> f32 {
        x.sqrt()
    }

    pub fn sin(deg: f32) -> f32 {
        (deg * DEG_TO_RAD).sin()
    }

    pub fn cos(deg: f32) -> f32 {
        (deg * DEG_TO_RAD).cos()
    }

    pub fn atan2(y: f32, x: f32) -> f32 {
        f32::atan2(y, x) * RAD_TO_DEG
    }

    pub fn random_seed(seed: u32) {
        *RNG.lock().unwrap() = Xoshiro256StarStar::seed_from_u64(seed as u64);
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
        *PERLIN.lock().unwrap() = Perlin::new(seed);
    }

    pub fn noise(x: f32, y: f32, z: f32) -> f32 {
        PERLIN.lock().unwrap().get([x as f64, y as f64, z as f64]) as f32
    }
}
