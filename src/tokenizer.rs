/// ref: https://github.com/huggingface/evaluate/blob/main/metrics/bleu/tokenizer_13a.py
use uluru::LRUCache;


pub trait Tokenizer {
    fn signature(&self) -> String;
    fn tokenize(&mut self, line: &str) -> Vec<String>;
}


#[derive(Debug)]
pub struct StringTokens {
    string: String,
    tokens: Vec<String>
}

// TODO: size change from 10240 to 65534 will have memory error
type StringTokensCache = LRUCache<StringTokens, 10240>;

#[derive(Debug)]
pub struct TokenizerRegex {
    // cache: HashMap<String, Vec<String>>
    cache: StringTokensCache
}


impl TokenizerRegex {
    pub fn new() -> Self {
        Self {cache: StringTokensCache::default()}
    }
}

impl Tokenizer for TokenizerRegex {
    fn signature(&self) -> String {
        "re".to_string()
    }

    fn tokenize(&mut self, line: &str) -> Vec<String> {
        let line_sting = line.to_string();
        match self.cache.find(|x| x.string == line_sting) {
            Some(item) => item.tokens.clone(),
            None => {
                let res: Vec<String> = line_sting.split(' ').map(|x| x.to_string()).collect();
                self.cache.insert(StringTokens{string: line_sting.clone(), tokens: res.clone()});
                res.clone()
            }
        }
    }
}


#[cfg(test)]
mod test {
    use crate::tokenizer;
    use crate::tokenizer::Tokenizer;

    #[test]
    fn test_tokenize_regex() {
        let mut tokenizer_regex = tokenizer::TokenizerRegex::new();
        let line = "Hello, World!";
        let res = tokenizer_regex.tokenize(line);
        assert_eq!(res, vec!["Hello,", "World!"])
    }
}