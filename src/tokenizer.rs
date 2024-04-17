/// ref: https://github.com/huggingface/evaluate/blob/main/metrics/bleu/tokenizer_13a.py
use cached::proc_macro::cached;
use lazy_static::lazy_static;
use regex::Regex;


lazy_static! {
    static ref TOKEN_REGEXES: [Regex; 4] = [
        Regex::new(r"pattern1").unwrap(),
        Regex::new(r"pattern2").unwrap(),
        Regex::new(r"pattern1").unwrap(),
        Regex::new(r"pattern1").unwrap(),
    ];
}

pub trait Tokenizer {
    fn signature(&self) -> String;
    fn tokenize(&self, line: &str) -> Vec<String>;
}


#[derive(Debug)]
pub struct TokenizerRegex {
    signature: String,
}


impl TokenizerRegex {
    pub fn new() -> Self {
        Self {signature: "re".to_string()}
    }
}


#[cached(size=65536)]
fn regex_tokenize_cache(line: String) -> Vec<String>  {
    // TODO: real regex process
    line.split(' ').map(|x| x.to_string()).collect()
}

impl Tokenizer for TokenizerRegex {
    fn signature(&self) -> String {
        self.signature.clone()
    }
    fn tokenize(&self, line: &str) -> Vec<String> {
        regex_tokenize_cache(line.to_string())
    }
}


#[cfg(test)]
mod test {
    use crate::tokenizer;
    use crate::tokenizer::Tokenizer;

    #[test]
    fn test_tokenize_regex() {
        let tokenizer_regex = tokenizer::TokenizerRegex::new();
        let line = "Hello, World!";
        let res = tokenizer_regex.tokenize(line);
        assert_eq!(res, vec!["Hello,", "World!"])
    }
}