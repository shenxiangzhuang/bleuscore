use bleuscore::tokenizer::{Tokenizer, Tokenizer13a, TokenizerRegex};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

/// Benchmark Tokenizer13a with short text
#[divan::bench]
fn tokenizer_13a_short() {
    let tokenizer = Tokenizer13a::new();
    let line = "Hello, &quot;World!<skipped>";
    
    tokenizer.tokenize(line);
}

/// Benchmark Tokenizer13a with medium text
#[divan::bench]
fn tokenizer_13a_medium() {
    let tokenizer = Tokenizer13a::new();
    let line = "/usr/sbin/sendmail - 0 errors, 12 warnings";
    
    tokenizer.tokenize(line);
}

/// Benchmark Tokenizer13a with long text
#[divan::bench]
fn tokenizer_13a_long() {
    let tokenizer = Tokenizer13a::new();
    let line = "The quick brown fox jumps over the lazy dog. This is a sample sentence for testing tokenization with longer text sequences.";
    
    tokenizer.tokenize(line);
}

/// Benchmark Tokenizer13a with various text lengths
#[divan::bench(args = [10, 50, 100, 200, 500])]
fn tokenizer_13a_varying_length(word_count: usize) -> Vec<String> {
    let tokenizer = Tokenizer13a::new();
    let line = "word ".repeat(word_count);
    
    tokenizer.tokenize(&line)
}

/// Benchmark TokenizerRegex with short text
#[divan::bench]
fn tokenizer_regex_short() {
    let tokenizer = TokenizerRegex::new();
    let line = "Hello, World!";
    
    tokenizer.tokenize(line);
}

/// Benchmark TokenizerRegex with medium text
#[divan::bench]
fn tokenizer_regex_medium() {
    let tokenizer = TokenizerRegex::new();
    let line = "/usr/sbin/sendmail - 0 errors, 12 warnings";
    
    tokenizer.tokenize(line);
}

/// Benchmark TokenizerRegex with long text
#[divan::bench]
fn tokenizer_regex_long() {
    let tokenizer = TokenizerRegex::new();
    let line = "The quick brown fox jumps over the lazy dog. This is a sample sentence for testing tokenization with longer text sequences.";
    
    tokenizer.tokenize(line);
}

/// Compare both tokenizers
#[divan::bench(types = [Tokenizer13a, TokenizerRegex])]
fn tokenizer_comparison<T>() 
where
    T: Tokenizer + Default,
{
    let tokenizer = T::default();
    let line = "Hello, World! This is a test.";
    
    tokenizer.tokenize(line);
}

/// Benchmark tokenizer with special characters
#[divan::bench]
fn tokenizer_special_chars() {
    let tokenizer = Tokenizer13a::new();
    let line = "Hello, &quot;World&amp;Rust&lt;Code&gt;!<skipped>";
    
    tokenizer.tokenize(line);
}
