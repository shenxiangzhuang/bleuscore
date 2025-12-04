use bleuscore::ngram::get_token_ngram_counter;
use divan::black_box;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

/// Benchmark n-gram counter with short token sequence
#[divan::bench(sample_count = 1000, sample_size = 100)]
fn ngram_short() {
    let tokens: Vec<String> = vec!["a".to_string(), "b".to_string()];
    let max_order = 4;

    black_box(get_token_ngram_counter(black_box(&tokens), black_box(max_order)));
}

/// Benchmark n-gram counter with medium token sequence
#[divan::bench(sample_count = 1000, sample_size = 100)]
fn ngram_medium() {
    let tokens: Vec<String> = vec![
        "a".to_string(),
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
    ];
    let max_order = 4;

    black_box(get_token_ngram_counter(black_box(&tokens), black_box(max_order)));
}

/// Benchmark n-gram counter with long token sequence
#[divan::bench(sample_count = 500, sample_size = 100)]
fn ngram_long() {
    let tokens: Vec<String> = vec![
        "the".to_string(),
        "quick".to_string(),
        "brown".to_string(),
        "fox".to_string(),
        "jumps".to_string(),
        "over".to_string(),
        "the".to_string(),
        "lazy".to_string(),
        "dog".to_string(),
    ];
    let max_order = 4;

    black_box(get_token_ngram_counter(black_box(&tokens), black_box(max_order)));
}

/// Benchmark n-gram counter with different max_order values
#[divan::bench(args = [1, 2, 3, 4, 5, 6, 8, 10], sample_count = 500, sample_size = 50)]
fn ngram_max_order(max_order: usize) {
    let tokens: Vec<String> = vec![
        "the".to_string(),
        "quick".to_string(),
        "brown".to_string(),
        "fox".to_string(),
        "jumps".to_string(),
        "over".to_string(),
        "the".to_string(),
        "lazy".to_string(),
        "dog".to_string(),
    ];

    black_box(get_token_ngram_counter(black_box(&tokens), black_box(max_order)));
}

/// Benchmark n-gram counter with different token sequence lengths
#[divan::bench(args = [5, 10, 20, 50, 100, 200], sample_count = 200, sample_size = 50)]
fn ngram_varying_length(token_count: usize) {
    let tokens: Vec<String> = (0..token_count)
        .map(|i| format!("word{}", i % 10))
        .collect();
    let max_order = 4;

    black_box(get_token_ngram_counter(black_box(&tokens), black_box(max_order)));
}

/// Benchmark n-gram counter with repeated tokens (high overlap)
#[divan::bench(sample_count = 500, sample_size = 100)]
fn ngram_high_overlap() {
    let tokens: Vec<String> = vec!["a".to_string(); 20];
    let max_order = 4;

    black_box(get_token_ngram_counter(black_box(&tokens), black_box(max_order)));
}

/// Benchmark n-gram counter with unique tokens (no overlap)
#[divan::bench(sample_count = 500, sample_size = 100)]
fn ngram_no_overlap() {
    let tokens: Vec<String> = (0..20).map(|i| format!("word{}", i)).collect();
    let max_order = 4;

    black_box(get_token_ngram_counter(black_box(&tokens), black_box(max_order)));
}

/// Benchmark n-gram counter with realistic sentence-like data
#[divan::bench(sample_count = 500, sample_size = 100)]
fn ngram_realistic() {
    let tokens: Vec<String> = vec![
        "the",
        "quick",
        "brown",
        "fox",
        "jumps",
        "over",
        "the",
        "lazy",
        "dog",
        "and",
        "the",
        "dog",
        "was",
        "sleeping",
        "peacefully",
        "under",
        "the",
        "tree",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let max_order = 4;

    black_box(get_token_ngram_counter(black_box(&tokens), black_box(max_order)));
}
