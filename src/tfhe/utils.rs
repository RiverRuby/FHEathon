use rand::Rng;
use rand_distr::{Distribution, Normal};

pub fn uniform_sample_int32(size: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..size)
        .map(|_| rng.gen_range(i32::MIN..=i32::MAX))
        .collect()
}

pub fn gaussian_sample_int32(std: f64, size: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(0.0, std).unwrap();
    (0..size)
        .map(|_| (i32::MAX as f64 * normal.sample(&mut rng)) as i32)
        .collect()
}

pub fn encode(message: i32) -> i32 {
    assert!(message >= -4 && message < 4);
    message.wrapping_mul(1 << 29)
}

pub fn decode(message: i32) -> i32 {
    let d = (message as f64 / (1 << 29) as f64).round() as i32;
    ((d + 4) % 8) - 4
}
