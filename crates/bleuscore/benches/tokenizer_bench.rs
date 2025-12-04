use bleuscore::tokenizer::{Tokenizer, Tokenizer13a, TokenizerRegex};
use divan::black_box;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

/// Benchmark Tokenizer13a with short text
#[divan::bench(sample_count = 1000, sample_size = 100)]
fn tokenizer_13a_short() {
    let tokenizer = Tokenizer13a::new();
    let line = "Hello, &quot;World!<skipped>";

    black_box(tokenizer.tokenize(black_box(line)));
}

/// Benchmark Tokenizer13a with medium text
#[divan::bench(sample_count = 1000, sample_size = 100)]
fn tokenizer_13a_medium() {
    let tokenizer = Tokenizer13a::new();
    let line = "/usr/sbin/sendmail - 0 errors, 12 warnings";

    black_box(tokenizer.tokenize(black_box(line)));
}

/// Benchmark Tokenizer13a with long text
#[divan::bench(sample_count = 500, sample_size = 100)]
fn tokenizer_13a_long() {
    let tokenizer = Tokenizer13a::new();
    let line = "The quick brown fox jumps over the lazy dog. This is a sample sentence for testing tokenization with longer text sequences.";

    black_box(tokenizer.tokenize(black_box(line)));
}

/// Benchmark Tokenizer13a with various text lengths
#[divan::bench(args = [10, 50, 100, 200, 500], sample_count = 500, sample_size = 50)]
fn tokenizer_13a_varying_length(word_count: usize) -> Vec<String> {
    let tokenizer = Tokenizer13a::new();
    let line = "word ".repeat(word_count);

    black_box(tokenizer.tokenize(black_box(&line)))
}

/// Benchmark TokenizerRegex with short text
#[divan::bench(sample_count = 1000, sample_size = 100)]
fn tokenizer_regex_short() {
    let tokenizer = TokenizerRegex::new();
    let line = "Hello, World!";

    black_box(tokenizer.tokenize(black_box(line)));
}

/// Benchmark TokenizerRegex with medium text
#[divan::bench(sample_count = 1000, sample_size = 100)]
fn tokenizer_regex_medium() {
    let tokenizer = TokenizerRegex::new();
    let line = "/usr/sbin/sendmail - 0 errors, 12 warnings";

    black_box(tokenizer.tokenize(black_box(line)));
}

/// Benchmark TokenizerRegex with long text
#[divan::bench(sample_count = 500, sample_size = 100)]
fn tokenizer_regex_long() {
    let tokenizer = TokenizerRegex::new();
    let line = "The quick brown fox jumps over the lazy dog. This is a sample sentence for testing tokenization with longer text sequences.";

    black_box(tokenizer.tokenize(black_box(line)));
}

/// Compare both tokenizers
#[divan::bench(types = [Tokenizer13a, TokenizerRegex], sample_count = 1000, sample_size = 100)]
fn tokenizer_comparison<T>()
where
    T: Tokenizer + Default,
{
    let tokenizer = T::default();
    let line = "Hello, World! This is a test.";

    black_box(tokenizer.tokenize(black_box(line)));
}

/// Benchmark tokenizer with special characters
#[divan::bench(sample_count = 1000, sample_size = 100)]
fn tokenizer_special_chars() {
    let tokenizer = Tokenizer13a::new();
    let line = "Hello, &quot;World&amp;Rust&lt;Code&gt;!<skipped>";

    black_box(tokenizer.tokenize(black_box(line)));
}
