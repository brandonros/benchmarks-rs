use benchmarks::base58;

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

fn bench_bs58_encode(c: &mut Criterion) {
    let public_key_bytes: [u8; 32] = hex::decode("0af764c1b6133a3a0abd7ef9c853791b687ce1e235f9dc8466d886da314dbea7")
        .unwrap()
        .as_slice()
        .try_into()
        .unwrap();
    
    let mut group = c.benchmark_group("base58");
    
    // Configure sample size and measurement time
    group.sample_size(1000);
    group.measurement_time(std::time::Duration::from_secs(10));
    group.throughput(Throughput::Bytes(32));
    
    group.bench_function("base58_encode32", |b| {
        b.iter(|| {
            let mut bs58_encoded_public_key = [0u8; 64];
            let _encoded_len = black_box(base58::base58_encode32(&public_key_bytes, &mut bs58_encoded_public_key));
        })
    });
    
    group.finish();
}

criterion_group!(benches, bench_bs58_encode);
criterion_main!(benches);
