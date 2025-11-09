fn main() {
    // Run registered benchmarks.
    divan::main();
}

/// Benchmark BLEU score computation with single prediction-reference pair
#[divan::bench]
fn bleu_single() {
    let max_order: usize = 4;
    let smooth: bool = true;
    let references: Vec<Vec<String>> = vec![vec!["Hello, World!".to_string()]];
    let predictions: Vec<String> = vec!["Yellow, World!".to_string()];

    bleuscore::compute_score(&references, &predictions, max_order, smooth);
}

/// Benchmark BLEU score computation with different batch sizes
#[divan::bench(args = [10, 50, 100, 500, 1000])]
fn bleu_batch(n: usize) {
    let max_order: usize = 4;
    let smooth: bool = true;

    let mut references: Vec<Vec<String>> = Vec::with_capacity(n);
    references.resize_with(n, || vec!["Hello, World!".to_string()]);

    let mut predictions: Vec<String> = Vec::with_capacity(n);
    predictions.resize_with(n, || "Yellow, World!".to_string());

    bleuscore::compute_score(&references, &predictions, max_order, smooth);
}

/// Benchmark BLEU score computation with different max_order values
#[divan::bench(args = [1, 2, 3, 4, 5, 6])]
fn bleu_max_order(max_order: usize) {
    let smooth: bool = true;
    let references: Vec<Vec<String>> = vec![vec!["Hello, World!".to_string()]];
    let predictions: Vec<String> = vec!["Yellow, World!".to_string()];

    bleuscore::compute_score(&references, &predictions, max_order, smooth);
}

/// Benchmark BLEU score with/without smoothing
#[divan::bench(args = [true, false])]
fn bleu_smoothing(smooth: bool) {
    let max_order: usize = 4;
    let references: Vec<Vec<String>> = vec![vec!["Hello, World!".to_string()]];
    let predictions: Vec<String> = vec!["Yellow, World!".to_string()];

    bleuscore::compute_score(&references, &predictions, max_order, smooth);
}

/// Benchmark BLEU score with longer texts
#[divan::bench]
fn bleu_long_text() {
    let max_order: usize = 4;
    let smooth: bool = true;
    let references: Vec<Vec<String>> = vec![vec![
        "The quick brown fox jumps over the lazy dog. This is a sample sentence for testing BLEU score calculation with longer text sequences.".to_string()
    ]];
    let predictions: Vec<String> = vec![
        "The quick brown fox leaps over the lazy dog. This is a test sentence for evaluating BLEU score computation with longer text sequences.".to_string()
    ];

    bleuscore::compute_score(&references, &predictions, max_order, smooth);
}
