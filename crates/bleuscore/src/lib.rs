/*!
bleuscore is a [BLEU](https://en.wikipedia.org/wiki/BLEU) score calculator written in pure rust.

# Install:

The crate is called `bleuscore` and you can depend on it via cargo:

```ini
[dependencies]
bleuscore = "*"
```

# Features:

- Tokenized BLEU score calculation like
[huggingface evaluate](https://github.com/huggingface/evaluate/blob/main/metrics/bleu/bleu.py).


# Basic usage:

```rust
use bleuscore::compute_score;

// get the references and prediction data:
let references: Vec<Vec<String>> = vec![vec!["Hello, World!".to_string()]];
let predictions: Vec<String> = vec!["Yellow, World!".to_string()];

// set the parameters:
let max_order: usize = 4;
let smooth: bool = true;

// calculate the BLEU score:
let res = compute_score(&references, &predictions, max_order, smooth);
println!("result: {:?}", res);
// result: BleuScore { bleu: 0.668740304976422, precisions: [0.8, 0.75, 0.6666666666666666, 0.5],
// brevity_penalty: 1.0, length_ratio: 1.0, translation_length: 4, reference_length: 4 }
```
!*/
#![feature(test)]
extern crate test;

pub mod bleu;
pub mod ngram;
pub mod tokenizer;
pub use bleu::compute_score;
