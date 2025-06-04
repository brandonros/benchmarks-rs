use benchmarks::ed25519;

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

fn bench_ed25519(c: &mut Criterion) {
    let private_key_bytes: [u8; 64] = hex::decode("0af764c1b6133a3a0abd7ef9c853791b687ce1e235f9dc8466d886da314dbea70af764c1b6133a3a0abd7ef9c853791b687ce1e235f9dc8466d886da314dbea7")
        .unwrap()
        .as_slice()
        .try_into()
        .unwrap();

    let mut group = c.benchmark_group("ed25519");
    
    // Configure sample size and measurement time
    group.sample_size(1000);
    group.measurement_time(std::time::Duration::from_secs(10));
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("ed25519_derive_public_key", |b| {
        b.iter(|| {
            
            let _ = black_box(ed25519::ed25519_derive_public_key(black_box(&private_key_bytes)));
        })
    });
    
    group.finish();
}

criterion_group!(benches, bench_ed25519);
criterion_main!(benches);
