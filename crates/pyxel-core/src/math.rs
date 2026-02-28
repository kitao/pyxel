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

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-5;

    // ceil / floor

    #[test]
    fn test_ceil() {
        assert_eq!(Pyxel::ceil(1.1), 2);
        assert_eq!(Pyxel::ceil(1.0), 1);
        assert_eq!(Pyxel::ceil(-1.1), -1);
        assert_eq!(Pyxel::ceil(0.0), 0);
    }

    #[test]
    fn test_floor() {
        assert_eq!(Pyxel::floor(1.9), 1);
        assert_eq!(Pyxel::floor(1.0), 1);
        assert_eq!(Pyxel::floor(-1.1), -2);
        assert_eq!(Pyxel::floor(0.0), 0);
    }

    // sqrt

    #[test]
    fn test_sqrt() {
        assert!((Pyxel::sqrt(4.0) - 2.0).abs() < EPSILON);
        assert!((Pyxel::sqrt(0.0) - 0.0).abs() < EPSILON);
        assert!((Pyxel::sqrt(2.0) - std::f32::consts::SQRT_2).abs() < EPSILON);
    }

    // Trigonometric functions (degrees)

    #[test]
    fn test_sin() {
        assert!((Pyxel::sin(0.0) - 0.0).abs() < EPSILON);
        assert!((Pyxel::sin(90.0) - 1.0).abs() < EPSILON);
        assert!((Pyxel::sin(180.0) - 0.0).abs() < EPSILON);
        assert!((Pyxel::sin(270.0) - (-1.0)).abs() < EPSILON);
        assert!((Pyxel::sin(30.0) - 0.5).abs() < EPSILON);
    }

    #[test]
    fn test_cos() {
        assert!((Pyxel::cos(0.0) - 1.0).abs() < EPSILON);
        assert!((Pyxel::cos(90.0) - 0.0).abs() < EPSILON);
        assert!((Pyxel::cos(180.0) - (-1.0)).abs() < EPSILON);
        assert!((Pyxel::cos(60.0) - 0.5).abs() < EPSILON);
    }

    #[test]
    fn test_atan2() {
        assert!((Pyxel::atan2(0.0, 1.0) - 0.0).abs() < EPSILON);
        assert!((Pyxel::atan2(1.0, 0.0) - 90.0).abs() < EPSILON);
        assert!((Pyxel::atan2(0.0, -1.0) - 180.0).abs() < EPSILON);
        assert!((Pyxel::atan2(-1.0, 0.0) - (-90.0)).abs() < EPSILON);
    }

    #[test]
    fn test_sin_cos_identity() {
        for deg in [0.0, 30.0, 45.0, 60.0, 90.0, 135.0, 180.0, 270.0, 359.0] {
            let s = Pyxel::sin(deg);
            let c = Pyxel::cos(deg);
            assert!((s * s + c * c - 1.0).abs() < EPSILON, "deg={deg}");
        }
    }

    // Random

    #[test]
    fn test_seeded_random_is_reproducible() {
        Pyxel::random_seed(42);
        let a: Vec<i32> = (0..10).map(|_| Pyxel::random_int(0, 100)).collect();
        Pyxel::random_seed(42);
        let b: Vec<i32> = (0..10).map(|_| Pyxel::random_int(0, 100)).collect();
        assert_eq!(a, b);
    }

    #[test]
    fn test_random_int_range() {
        Pyxel::random_seed(0);
        for _ in 0..100 {
            let v = Pyxel::random_int(5, 10);
            assert!((5..=10).contains(&v));
        }
    }

    #[test]
    fn test_random_int_swapped_range() {
        Pyxel::random_seed(0);
        let v = Pyxel::random_int(10, 5);
        assert!((5..=10).contains(&v));
    }

    #[test]
    fn test_random_float_range() {
        Pyxel::random_seed(0);
        for _ in 0..100 {
            let v = Pyxel::random_float(0.0, 1.0);
            assert!((0.0..=1.0).contains(&v));
        }
    }

    // Noise

    #[test]
    fn test_noise_range() {
        Pyxel::noise_seed(0);
        for i in 0..20 {
            let v = Pyxel::noise(i as f32 * 0.1, 0.0, 0.0);
            assert!((-1.0..=1.0).contains(&v), "noise value {v} out of range");
        }
    }

    #[test]
    fn test_noise_seed_reproducible() {
        Pyxel::noise_seed(42);
        let a = Pyxel::noise(1.0, 2.0, 3.0);
        Pyxel::noise_seed(42);
        let b = Pyxel::noise(1.0, 2.0, 3.0);
        assert_eq!(a, b);
    }
}
