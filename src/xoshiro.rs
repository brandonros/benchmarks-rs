use rand_core::{SeedableRng, RngCore};
use rand_xoshiro::{Xoroshiro128StarStar, Xoroshiro128PlusPlus, Xoshiro256PlusPlus, Xoshiro256StarStar};

pub fn generate_random_private_key_xoroshiro128starstar(thread_idx: usize, rng_seed: u64) -> [u8; 32] {
    let mut private_key = [0u8; 32];
    let mut rng = Xoroshiro128StarStar::seed_from_u64(rng_seed ^ (thread_idx as u64));
    rng.fill_bytes(&mut private_key);
    private_key
}

pub fn generate_random_private_key_xoroshiro128plusplus(thread_idx: usize, rng_seed: u64) -> [u8; 32] {
    let mut private_key = [0u8; 32];
    let mut rng = Xoroshiro128PlusPlus::seed_from_u64(rng_seed ^ (thread_idx as u64));
    rng.fill_bytes(&mut private_key);
    private_key
}

pub fn generate_random_private_key_xoshiro256plusplus(thread_idx: usize, rng_seed: u64) -> [u8; 32] {
    let mut private_key = [0u8; 32];
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(rng_seed ^ (thread_idx as u64));
    rng.fill_bytes(&mut private_key);
    private_key
}

pub fn generate_random_private_key_xoshiro256starstar(thread_idx: usize, rng_seed: u64) -> [u8; 32] {
    let mut private_key = [0u8; 32];
    let mut rng = Xoshiro256StarStar::seed_from_u64(rng_seed ^ (thread_idx as u64));
    rng.fill_bytes(&mut private_key);
    private_key
}
