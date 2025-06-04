use benchmarks::xoroshiro;

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

fn bench_xoroshiro(c: &mut Criterion) {
    let mut group = c.benchmark_group("xoroshiro");
    
    // Configure sample size and measurement time
    group.sample_size(1000);
    group.measurement_time(std::time::Duration::from_secs(10));
    group.throughput(Throughput::Bytes(32));
    
    group.bench_function("generate_random_private_key", |b| {
        b.iter(|| {
            let thread_idx = 35353453;
            let rng_seed = 12345678;
            let _private_key = black_box(xoroshiro::generate_random_private_key(thread_idx, rng_seed));
        })
    });
    
    group.finish();
}

criterion_group!(benches, bench_xoroshiro);
criterion_main!(benches);
