use benchmarks::xoshiro;

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

fn bench_xoshiro(c: &mut Criterion) {
    let mut group = c.benchmark_group("xoshiro");
    
    // Configure sample size and measurement time
    group.sample_size(1000);
    group.measurement_time(std::time::Duration::from_secs(10));
    group.throughput(Throughput::Bytes(32));
    
    group.bench_function("xoshiro", |b| {
        b.iter(|| {
            let thread_idx = 0;
            let rng_seed = 12345678;
            let _private_key = black_box(xoshiro::generate_random_private_key(thread_idx, rng_seed));
        })
    });
    
    group.finish();
}

criterion_group!(benches, bench_xoshiro);
criterion_main!(benches);
