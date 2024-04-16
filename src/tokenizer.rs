/// ref: https://github.com/huggingface/evaluate/blob/main/metrics/bleu/tokenizer_13a.py

use std::collections::HashMap;

pub trait Tokenizer<'a> {
    fn signature(&self) -> &str;
    fn tokenize(&'a mut self, line: &'a str) -> Vec<String>;
}

#[derive(Debug)]
pub struct TokenizerRegex<'a> {
    cache: HashMap<&'a str, Vec<&'a str>>
}


impl TokenizerRegex<'_> {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new()
        }
    }

}

impl<'a> Tokenizer<'a> for TokenizerRegex<'a> {
    fn signature(&self) -> &str {
        "re"
    }

    fn tokenize(&'a mut self, line: &'a str) -> Vec<String> {
        if !self.cache.contains_key(line) {
            // todo regex
            let res: Vec<&str> = line.split(' ').collect();
            self.cache.insert(line, res.clone());
        }
        match self.cache.get(&line) {
            None => Vec::new(),
            Some(cache_res) => cache_res.iter().map(|&x| x.to_string()).collect()
        }
    }
}
