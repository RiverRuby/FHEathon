use super::utils::{decode, encode, gaussian_sample_int32, uniform_sample_int32};
use rand::Rng;

#[derive(Clone)]
pub struct LweConfig {
    // size of the LWE encryption key
    pub dimension: i32,

    // standard deviation of the encryption noise
    pub noise_std: f64,
}

#[derive(Debug, Clone)]
pub struct LwePlaintext {
    message: i32,
}

#[derive(Clone)]
pub struct LweCiphertext {
    config: LweConfig,
    a: Vec<i32>,
    b: i32,
}

#[derive(Clone)]
pub struct LweEncryptionKey {
    config: LweConfig,
    key: Vec<i32>,
}

pub fn generate_lwe_key(config: LweConfig) -> LweEncryptionKey {
    let mut rng = rand::thread_rng(); // Add this line
    LweEncryptionKey {
        config: config.clone(),
        key: (0..config.dimension).map(|_| rng.gen_range(0..2)).collect(),
    }
}

pub fn lwe_encode(message: i32) -> LwePlaintext {
    LwePlaintext {
        message: encode(message),
    }
}

pub fn lwe_decode(plaintext: LwePlaintext) -> i32 {
    decode(plaintext.message)
}

pub fn lwe_encrypt(plaintext: LwePlaintext, key: LweEncryptionKey) -> LweCiphertext {
    let a = uniform_sample_int32(key.config.dimension);
    let noise = gaussian_sample_int32(key.config.noise_std, 1);

    // b = <a, key> + message + noise
    let b = (a.iter())
        .zip(key.key.iter())
        .fold(plaintext.message, |acc, (a, key)| {
            acc.wrapping_add(a.wrapping_mul(*key))
        })
        .wrapping_add(noise[0]);

    LweCiphertext {
        config: key.config.clone(),
        a,
        b,
    }
}

pub fn lwe_decrypt(ciphertext: LweCiphertext, key: LweEncryptionKey) -> LwePlaintext {
    let message = (ciphertext.a.iter())
        .zip(key.key.iter())
        .fold(ciphertext.b, |acc, (a, key)| {
            acc.wrapping_sub(a.wrapping_mul(*key))
        });

    LwePlaintext { message }
}

pub fn lwe_add(ciphertext_left: LweCiphertext, ciphertext_right: LweCiphertext) -> LweCiphertext {
    let a = (ciphertext_left.a.iter())
        .zip(ciphertext_right.a.iter())
        .map(|(a, b)| a.wrapping_add(*b))
        .collect();

    let b = ciphertext_left.b.wrapping_add(ciphertext_right.b);

    LweCiphertext {
        config: ciphertext_left.config.clone(),
        a,
        b,
    }
}

pub fn lwe_sub(ciphertext_left: LweCiphertext, ciphertext_right: LweCiphertext) -> LweCiphertext {
    let a = (ciphertext_left.a.iter())
        .zip(ciphertext_right.a.iter())
        .map(|(a, b)| a.wrapping_sub(*b))
        .collect();

    let b = ciphertext_left.b.wrapping_sub(ciphertext_right.b);

    LweCiphertext {
        config: ciphertext_left.config.clone(),
        a,
        b,
    }
}

pub fn lwe_plaintext_multiply(c: i32, ciphertext: LweCiphertext) -> LweCiphertext {
    let a = ciphertext.a.iter().map(|a| a.wrapping_mul(c)).collect();
    let b = ciphertext.b.wrapping_mul(c);

    LweCiphertext {
        config: ciphertext.config.clone(),
        a,
        b,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tfhe::config::LWE_CONFIG;

    /**
     * Test LWE encryption and decryption and ensure noise is centered at 0 and
     * has a standard deviation close to the configured noise standard deviation.
     */
    #[test]
    fn test_lwe() {
        let config = LWE_CONFIG;
        let key = generate_lwe_key(config.clone());

        let num_trials = 1000;
        let mut noise_samples = Vec::with_capacity(num_trials);

        for _ in 0..num_trials {
            let plaintext = lwe_encode(rand::thread_rng().gen_range(-4..4));
            let ciphertext = lwe_encrypt(plaintext.clone(), key.clone());
            let decrypted = lwe_decrypt(ciphertext, key.clone());
            assert_eq!(lwe_decode(plaintext.clone()), lwe_decode(decrypted.clone()));

            let noise = decrypted.message.wrapping_sub(plaintext.message);
            noise_samples.push(noise);
        }

        let avg_noise = noise_samples.iter().sum::<i32>() as f64 / num_trials as f64;
        let variance = noise_samples
            .iter()
            .map(|&x| (x as f64 - avg_noise).powi(2))
            .sum::<f64>()
            / num_trials as f64;
        let std_dev = variance.sqrt();

        println!("Average noise: {}", avg_noise);
        println!("Standard deviation of noise: {}", std_dev);
        println!(
            "Configured noise std: {}",
            config.noise_std * 2_f64.powi(31)
        );
    }

    /**
     * Test LWE addition and subtraction.
     */
    #[test]
    fn test_lwe_add_sub() {
        let config = LWE_CONFIG;
        let key = generate_lwe_key(config.clone());

        let plaintext1 = lwe_encode(1);
        let plaintext2 = lwe_encode(2);

        let ciphertext1 = lwe_encrypt(plaintext1.clone(), key.clone());
        let ciphertext2 = lwe_encrypt(plaintext2.clone(), key.clone());

        let ciphertext_add = lwe_add(ciphertext1.clone(), ciphertext2.clone());
        let ciphertext_sub = lwe_sub(ciphertext1.clone(), ciphertext2.clone());

        let decrypted_add = lwe_decrypt(ciphertext_add, key.clone());
        let decrypted_sub = lwe_decrypt(ciphertext_sub, key.clone());

        assert_eq!(lwe_decode(decrypted_add), 3);
        assert_eq!(lwe_decode(decrypted_sub), -1);
    }

    /**
     * Test LWE plaintext multiplication.
     */
    #[test]
    fn test_lwe_plaintext_multiply() {
        let config = LWE_CONFIG;
        let key = generate_lwe_key(config.clone());

        let plaintext = lwe_encode(1);
        let ciphertext = lwe_encrypt(plaintext.clone(), key.clone());

        let ciphertext_mult = lwe_plaintext_multiply(3, ciphertext.clone());
        let decrypted_mult = lwe_decrypt(ciphertext_mult, key.clone());

        assert_eq!(lwe_decode(decrypted_mult), 3);
    }
}
