/// End-to-end integration benchmarks testing the complete BLEU scoring pipeline
/// These benchmarks simulate real-world usage scenarios

fn main() {
    divan::main();
}

/// Benchmark complete pipeline: tokenization + n-gram + BLEU computation
#[divan::bench]
fn e2e_single_sentence() {
    let references: Vec<Vec<String>> = vec![vec![
        "The quick brown fox jumps over the lazy dog".to_string()
    ]];
    let predictions: Vec<String> = vec![
        "The fast brown fox leaps over the lazy dog".to_string()
    ];
    
    bleuscore::compute_score(&references, &predictions, 4, true);
}

/// Benchmark with multiple references per prediction
#[divan::bench]
fn e2e_multiple_references() {
    let references: Vec<Vec<String>> = vec![vec![
        "The quick brown fox jumps over the lazy dog".to_string(),
        "A fast brown fox leaps over a sleeping dog".to_string(),
        "The brown fox quickly jumps over the dog".to_string(),
    ]];
    let predictions: Vec<String> = vec![
        "The fast brown fox leaps over the lazy dog".to_string()
    ];
    
    bleuscore::compute_score(&references, &predictions, 4, true);
}

/// Benchmark realistic batch processing with varying sentence lengths
#[divan::bench(args = [10, 50, 100])]
fn e2e_batch_realistic(batch_size: usize) {
    let sample_references = vec![
        "The quick brown fox jumps over the lazy dog".to_string(),
        "Machine learning is transforming natural language processing".to_string(),
        "Artificial intelligence enables computers to understand human language".to_string(),
        "Deep learning models achieve state-of-the-art results in NLP tasks".to_string(),
        "Neural networks can learn complex patterns from data".to_string(),
    ];
    
    let sample_predictions = vec![
        "The fast brown fox leaps over the lazy dog".to_string(),
        "Machine learning transforms natural language processing".to_string(),
        "AI enables computers to understand human languages".to_string(),
        "Deep learning achieves great results in NLP".to_string(),
        "Neural nets learn complex patterns from data".to_string(),
    ];
    
    let mut references: Vec<Vec<String>> = Vec::with_capacity(batch_size);
    let mut predictions: Vec<String> = Vec::with_capacity(batch_size);
    
    for i in 0..batch_size {
        references.push(vec![sample_references[i % sample_references.len()].clone()]);
        predictions.push(sample_predictions[i % sample_predictions.len()].clone());
    }
    
    bleuscore::compute_score(&references, &predictions, 4, true);
}

/// Benchmark with very short texts (edge case)
#[divan::bench]
fn e2e_short_texts() {
    let references: Vec<Vec<String>> = vec![vec!["Hi".to_string()]];
    let predictions: Vec<String> = vec!["Hello".to_string()];
    
    bleuscore::compute_score(&references, &predictions, 4, true);
}

/// Benchmark with very long texts
#[divan::bench]
fn e2e_long_texts() {
    let references: Vec<Vec<String>> = vec![vec![
        "In the field of natural language processing, the Bilingual Evaluation Understudy (BLEU) score is a widely used metric for evaluating the quality of machine-translated text. \
        It works by comparing a candidate translation to one or more reference translations, computing precision scores for n-grams of various lengths. \
        The BLEU score ranges from 0 to 1, where 1 indicates a perfect match with the reference translations. \
        Despite its widespread adoption, BLEU has known limitations, including its inability to capture semantic meaning and its bias towards shorter translations.".to_string()
    ]];
    let predictions: Vec<String> = vec![
        "In natural language processing, BLEU is a common metric for evaluating machine translation quality. \
        It compares candidate translations to reference translations by computing precision scores for n-grams. \
        BLEU scores range from 0 to 1, with 1 being perfect. \
        Though widely used, BLEU has limitations like not capturing semantics and favoring shorter outputs.".to_string()
    ];
    
    bleuscore::compute_score(&references, &predictions, 4, true);
}

/// Benchmark perfect match scenario
#[divan::bench]
fn e2e_perfect_match() {
    let text = "The quick brown fox jumps over the lazy dog".to_string();
    let references: Vec<Vec<String>> = vec![vec![text.clone()]];
    let predictions: Vec<String> = vec![text];
    
    bleuscore::compute_score(&references, &predictions, 4, true);
}

/// Benchmark complete mismatch scenario
#[divan::bench]
fn e2e_complete_mismatch() {
    let references: Vec<Vec<String>> = vec![vec![
        "The quick brown fox jumps over the lazy dog".to_string()
    ]];
    let predictions: Vec<String> = vec![
        "Python programming language machine learning AI".to_string()
    ];
    
    bleuscore::compute_score(&references, &predictions, 4, true);
}

/// Benchmark with special characters and punctuation
#[divan::bench]
fn e2e_special_chars() {
    let references: Vec<Vec<String>> = vec![vec![
        r#"Hello, "World"! How are you? I'm fine, thanks & you?"#.to_string()
    ]];
    let predictions: Vec<String> = vec![
        r#"Hello, "World"! How are you? I am fine, thanks & you?"#.to_string()
    ];
    
    bleuscore::compute_score(&references, &predictions, 4, true);
}

/// Benchmark translation-like scenario with multiple languages characteristics
#[divan::bench]
fn e2e_translation_scenario() {
    // Simulate English to simplified English translation
    let references: Vec<Vec<String>> = vec![vec![
        "The committee has decided to postpone the meeting until next week.".to_string()
    ]];
    let predictions: Vec<String> = vec![
        "The committee decided to delay the meeting until next week.".to_string()
    ];
    
    bleuscore::compute_score(&references, &predictions, 4, true);
}

/// Benchmark with HTML entities (common in web-scraped data)
#[divan::bench]
fn e2e_html_entities() {
    let references: Vec<Vec<String>> = vec![vec![
        r#"&quot;Hello&quot; &amp; &lt;World&gt;"#.to_string()
    ]];
    let predictions: Vec<String> = vec![
        r#"&quot;Hello&quot; and &lt;World&gt;"#.to_string()
    ];
    
    bleuscore::compute_score(&references, &predictions, 4, true);
}
