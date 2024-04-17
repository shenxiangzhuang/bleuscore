/// ref: https://github.com/huggingface/evaluate/blob/main/metrics/bleu/tokenizer_13a.py

use std::collections::HashMap;

pub trait Tokenizer {
    fn signature(&self) -> String;
    fn tokenize(&mut self, line: &str) -> Vec<String>;
}

#[derive(Debug)]
pub struct TokenizerRegex {
    cache: HashMap<String, Vec<String>>
}


impl TokenizerRegex {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new()
        }
    }

}

impl Tokenizer for TokenizerRegex {
    fn signature(&self) -> String {
        "re".to_string()
    }

    fn tokenize(&mut self, line: &str) -> Vec<String> {
        let line_sting = line.to_string();
        if !self.cache.contains_key(&line_sting) {
            // todo regex
            let res: Vec<String> = line_sting.split(" ").map(|x| x.to_string()).collect();
            self.cache.insert(line_sting.clone(), res);
        }
        match self.cache.get(&line_sting) {
            None => Vec::new(),
            Some(cache_res) => cache_res.clone()
        }
    }
}
