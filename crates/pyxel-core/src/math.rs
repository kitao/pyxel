use std::cell::RefCell;
use std::f32::consts::PI;

use noise::{NoiseFn, Perlin};
use rand::{RngExt, SeedableRng};
use rand_xoshiro::Xoshiro256StarStar;

use crate::pyxel::Pyxel;

const DEG_TO_RAD: f32 = PI / 180.0;
const RAD_TO_DEG: f32 = 180.0 / PI;

thread_local! {
    static RNG: RefCell<Xoshiro256StarStar> = RefCell::new(new_rng());
    static PERLIN: RefCell<Perlin> = RefCell::new(new_perlin());
}

impl Pyxel {
    // Basic math

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

    // Random

    pub fn random_seed(seed: u32) {
        with_rng(|rng| *rng = Xoshiro256StarStar::seed_from_u64(seed as u64));
    }

    pub fn random_int(min: i32, max: i32) -> i32 {
        let (min, max) = if min < max { (min, max) } else { (max, min) };
        with_rng(|rng| rng.random_range(min..=max))
    }

    pub fn random_float(min: f32, max: f32) -> f32 {
        let (min, max) = if min < max { (min, max) } else { (max, min) };
        with_rng(|rng| rng.random_range(min..=max))
    }

    // Noise

    pub fn noise_seed(seed: u32) {
        with_perlin(|perlin| *perlin = Perlin::new(seed));
    }

    pub fn noise(x: f32, y: f32, z: f32) -> f32 {
        with_perlin(|perlin| perlin.get([x as f64, y as f64, z as f64]) as f32)
    }
}

// Helpers

fn new_rng() -> Xoshiro256StarStar {
    Xoshiro256StarStar::from_rng(&mut rand::rng())
}

fn with_rng<T>(f: impl FnOnce(&mut Xoshiro256StarStar) -> T) -> T {
    RNG.with(|rng| f(&mut rng.borrow_mut()))
}

fn new_perlin() -> Perlin {
    Perlin::new(rand::rng().random())
}

fn with_perlin<T>(f: impl FnOnce(&mut Perlin) -> T) -> T {
    PERLIN.with(|perlin| f(&mut perlin.borrow_mut()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_state_is_isolated_between_threads() {
        Pyxel::random_seed(123);
        let expected = Pyxel::random_int(i32::MIN, i32::MAX);

        Pyxel::random_seed(123);
        std::thread::spawn(|| {
            Pyxel::random_seed(999);
            Pyxel::random_int(i32::MIN, i32::MAX);
        })
        .join()
        .unwrap();

        assert_eq!(Pyxel::random_int(i32::MIN, i32::MAX), expected);
    }

    #[test]
    fn noise_state_is_isolated_between_threads() {
        const POINT: (f32, f32, f32) = (1.5, 2.5, 3.5);

        Pyxel::noise_seed(123);
        let expected = Pyxel::noise(POINT.0, POINT.1, POINT.2);

        Pyxel::noise_seed(123);
        std::thread::spawn(|| {
            Pyxel::noise_seed(999);
            Pyxel::noise(POINT.0, POINT.1, POINT.2);
        })
        .join()
        .unwrap();

        assert_eq!(Pyxel::noise(POINT.0, POINT.1, POINT.2), expected);
    }
}
