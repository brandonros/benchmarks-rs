fn main() {
    let public_key_bytes: [u8; 32] = hex::decode("0af764c1b6133a3a0abd7ef9c853791b687ce1e235f9dc8466d886da314dbea7").unwrap().as_slice().try_into().unwrap();
    let expected: [u8; 43] = hex::decode("6a6f7365413875757746426a58707558423879453233437845756d596758336a486251677753627166504c").unwrap().as_slice().try_into().unwrap();
    let num_iterations = 100_000_000;
    
    // Warmup to ensure stable measurements
    println!("Warming up...");
    for _ in 0..1000 {
        let mut bs58_encoded_public_key = [0u8; 64];
        let encoded_len = base58::base58_encode(&public_key_bytes, &mut bs58_encoded_public_key);
        let bs58_encoded_public_key = &bs58_encoded_public_key[0..encoded_len];
        assert_eq!(*bs58_encoded_public_key, expected);
    }
    
    println!("Running benchmark with {} iterations...", num_iterations);
    let start_time = std::time::Instant::now();
    
    for _ in 0..num_iterations {
        let mut bs58_encoded_public_key = [0u8; 64];
        let encoded_len = base58::base58_encode(&public_key_bytes, &mut bs58_encoded_public_key);
        let bs58_encoded_public_key = &bs58_encoded_public_key[0..encoded_len];
        // do not optimize out the bs58_encoded_public_key
        std::hint::black_box(&bs58_encoded_public_key);
    }
    
    let duration = start_time.elapsed();
    let duration_secs = duration.as_secs_f64();
    let ops_per_sec = num_iterations as f64 / duration_secs;
    
    println!("Results:");
    println!("  Total time: {:?}", duration);
    println!("  Time per operation: {:.2} ns", duration.as_nanos() as f64 / num_iterations as f64);
    println!("  Operations per second: {:.0}", ops_per_sec);
    println!("  Throughput: {:.2} MB/s", (ops_per_sec * 32.0) / (1024.0 * 1024.0)); // 32 bytes per operation
}

#[cfg(test)]
mod test {
    #[test]
    fn should_encode_correctly() {
        let public_key_bytes: [u8; 32] = hex::decode("0af764c1b6133a3a0abd7ef9c853791b687ce1e235f9dc8466d886da314dbea7").unwrap().as_slice().try_into().unwrap();

        // bs58 encode public key
        let mut bs58_encoded_public_key = [0u8; 64];
        let encoded_len = base58::base58_encode(&public_key_bytes, &mut bs58_encoded_public_key);
        let bs58_encoded_public_key = &bs58_encoded_public_key[0..encoded_len];
        let expected = hex::decode("6a6f7365413875757746426a58707558423879453233437845756d596758336a486251677753627166504c").unwrap();
        assert_eq!(*bs58_encoded_public_key, *expected);

        // utf8
        use std::string::String;
        let bs58_encoded_public_key_string = String::from_utf8(bs58_encoded_public_key.to_vec()).unwrap();
        let expected = "joseA8uuwFBjXpuXB8yE23CxEumYgX3jHbQgwSbqfPL";
        assert_eq!(bs58_encoded_public_key_string, expected);
    }
}
