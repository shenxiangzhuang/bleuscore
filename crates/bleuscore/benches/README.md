# BLEUScore Benchmarks

This directory contains performance benchmarks for the BLEUScore library using [Divan](https://github.com/nvzqz/divan) and [CodSpeed](https://codspeed.io/).

## Running Benchmarks

### Prerequisites

Install `cargo-codspeed`:
```bash
cargo install cargo-codspeed --locked
```

### Local Testing

Build the benchmarks:
```bash
cargo codspeed build -p bleuscore
```

Run all benchmarks:
```bash
cargo codspeed run
```

Run specific benchmark:
```bash
cargo codspeed run --bench bleu_bench
```

Run benchmarks matching a pattern:
```bash
cargo codspeed run "bleu_batch"
```

### Standard Divan Benchmarks

You can also run benchmarks without CodSpeed instrumentation:
```bash
cargo bench -p bleuscore
```

## CI Integration

Benchmarks are automatically run in CI via the `.github/workflows/codspeed.yml` workflow:
- On push to `master` branch
- On pull requests
- Can be triggered manually via `workflow_dispatch`

CodSpeed will automatically track performance changes and report regressions on pull requests.

## Benchmark Structure

Each benchmark file follows the Divan pattern:
```rust
fn main() {
    divan::main();
}

#[divan::bench]
fn my_benchmark() {
    // benchmark code
}

#[divan::bench(args = [1, 2, 3])]
fn parameterized_benchmark(n: usize) {
    // benchmark with different inputs
}
```

## Migration from Old Benchmarks

The old benchmarks using `#[cfg(feature = "nightly-test")]` have been removed. All benchmarks now use the modern Divan framework with CodSpeed compatibility.
