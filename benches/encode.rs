use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

fn bench_bs58_encode(c: &mut Criterion) {
    let public_key_bytes: [u8; 32] = hex::decode("0af764c1b6133a3a0abd7ef9c853791b687ce1e235f9dc8466d886da314dbea7")
        .unwrap()
        .as_slice()
        .try_into()
        .unwrap();
    
    let mut group = c.benchmark_group("bs58_encoding_custom");
    
    // Configure sample size and measurement time
    group.sample_size(1000);
    group.measurement_time(std::time::Duration::from_secs(10));
    group.throughput(Throughput::Bytes(32));
    
    group.bench_function("bs58_encode_custom", |b| {
        b.iter(|| {
            let mut bs58_encoded_public_key = [0u8; 64];
            let encoded_len = base58::base58_encode(black_box(&public_key_bytes), &mut bs58_encoded_public_key);
            let _ = &bs58_encoded_public_key[0..encoded_len];
        })
    });
    
    group.finish();
}

criterion_group!(benches, bench_bs58_encode);
criterion_main!(benches);
