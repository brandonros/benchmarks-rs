# benchmarks-rs
Different benchmarks in Rust

## How to test

```shell
cargo test
```

## How to run

```shell
cargo bench base58
cargo bench ed25519
cargo bench sha512
cargo bench xoroshiro
```

## Results

```
$ cargo bench
    Finished `bench` profile [optimized] target(s) in 0.04s
     Running unittests src/lib.rs (/Users/brandon/.cargo/target/release/deps/base58-d64ec8ec23e3ee64)

running 1 test
test test::should_encode_correctly ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/encode.rs (/Users/brandon/.cargo/target/release/deps/encode-3d2a6b7a76dea093)
Gnuplot not found, using plotters backend
bs58_encoding_custom/bs58_encode_custom
                        time:   [114.30 ns 114.34 ns 114.38 ns]
                        thrpt:  [266.82 MiB/s 266.90 MiB/s 267.00 MiB/s]
                 change:
                        time:   [-0.3048% +0.0246% +0.2981%] (p = 0.88 > 0.05)
                        thrpt:  [-0.2972% -0.0246% +0.3057%]
                        No change in performance detected.
Found 24 outliers among 1000 measurements (2.40%)
  18 (1.80%) low mild
  6 (0.60%) high severe
```
