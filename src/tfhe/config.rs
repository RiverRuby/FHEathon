use crate::tfhe::lwe::LweConfig;

/*
 * These constants are taken from Lattice Estimator.
 * https://github.com/malb/lattice-estimator
 */

const STD: f64 = 5.96046448e-8; // 2^-24

pub const LWE_CONFIG: LweConfig = LweConfig {
    dimension: 1024,
    noise_std: STD,
};
