use benchmarks::sha512;

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

fn bench_sha512(c: &mut Criterion) {
    let public_key_bytes: [u8; 32] = hex::decode("0af764c1b6133a3a0abd7ef9c853791b687ce1e235f9dc8466d886da314dbea7")
        .unwrap()
        .as_slice()
        .try_into()
        .unwrap();

    let mut group = c.benchmark_group("sha512");
    
    // Configure sample size and measurement time
    group.sample_size(1000);
    group.measurement_time(std::time::Duration::from_secs(10));
    group.throughput(Throughput::Bytes(32));
    
    group.bench_function("sha512_32bytes_from_bytes", |b| {
        b.iter(|| {
            
            let _ = black_box(sha512::sha512_32bytes_from_bytes(black_box(&public_key_bytes)));
        })
    });
    
    group.finish();
}

criterion_group!(benches, bench_sha512);
criterion_main!(benches);
