/// ref: https://github.com/huggingface/evaluate/blob/main/metrics/bleu/tokenizer_13a.py
use cached::proc_macro::cached;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX_ARRAY: [(Regex, &'static str); 4] = [
        (Regex::new(r"([\{-\~\[-\` -\&\(-\+\:-\@\/])").unwrap(),  r" $1 "),
        (Regex::new(r"([^0-9])([\.,])").unwrap(), r"$1 $2 "),
        (Regex::new(r"([\.,])([^0-9])").unwrap(), r" $1 $2"),
        (Regex::new(r"([0-9])(-)").unwrap(), r"$1 $2 "),
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
    let mut res = line;
    for &(ref re_capture, re_replace) in REGEX_ARRAY.iter() {
        res = re_capture.replace_all(&res, re_replace).to_string();
    }
    res.split_whitespace().map(|x| x.to_string()).collect()
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
        let mut line = "Hello, World!";
        let mut res = tokenizer_regex.tokenize(line);
        
        line = "/usr/sbin/sendmail - 0 errors, 12 warnings";
        res = tokenizer_regex.tokenize(line);
        assert_eq!(res, vec!["/", "usr", "/", "sbin", "/", "sendmail", "-", "0", "errors", ",", "12", "warnings"])
    }
}