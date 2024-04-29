use cached::proc_macro::cached;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref REGEX_ARRAY: [(Regex, &'static str); 4] = [
        (
            Regex::new(r"([\{-\~\[-\` -\&\(-\+\:-\@\/])").unwrap(),
            r" $1 "
        ),
        (Regex::new(r"([^0-9])([\.,])").unwrap(), r"$1 $2 "),
        (Regex::new(r"([\.,])([^0-9])").unwrap(), r" $1 $2"),
        (Regex::new(r"([0-9])(-)").unwrap(), r"$1 $2 "),
    ];
}

/// tokenize function is used to tokenize the strings
pub trait Tokenizer {
    fn signature(&self) -> &str;
    fn tokenize(&self, line: &str) -> Vec<String>;
}

/// Same implementation with [huggingface/sacrebleu](https://github.com/huggingface/evaluate/blob/main/metrics/bleu/tokenizer_13a.py)
#[derive(Debug)]
pub struct TokenizerRegex {
    pub signature: String,
}

impl Default for TokenizerRegex {
    fn default() -> Self {
        Self {
            signature: "re".to_string(),
        }
    }
}

impl TokenizerRegex {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cached(size = 65536)]
fn regex_tokenize_cache(line: String) -> Vec<String> {
    let mut res = line;
    for &(ref re_capture, re_replace) in REGEX_ARRAY.iter() {
        res = re_capture.replace_all(&res, re_replace).to_string();
    }
    res.split_whitespace().map(|x| x.to_string()).collect()
}

impl Tokenizer for TokenizerRegex {
    fn signature(&self) -> &str {
        &self.signature
    }
    fn tokenize(&self, line: &str) -> Vec<String> {
        regex_tokenize_cache(line.to_string())
    }
}

/// Same implementation with [huggingface/sacrebleu](https://github.com/huggingface/evaluate/blob/main/metrics/bleu/tokenizer_13a.py)
#[derive(Debug)]
pub struct Tokenizer13a {
    pub signature: String,
}

impl Default for Tokenizer13a {
    fn default() -> Self {
        Self {
            signature: "13a".to_string(),
        }
    }
}

impl Tokenizer13a {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cached(size = 65536)]
fn tokenize_13a_cache(line: String) -> Vec<String> {
    let mut res = line;
    res = res
        .replace("<skipped>", "")
        .replace("-\n", "")
        .replace('\n', " ");
    if res.contains('&') {
        res = res
            .replace("&quot;", "\"")
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">");
    }
    TokenizerRegex::new().tokenize(&format!(" {res} "))
}

impl Tokenizer for Tokenizer13a {
    fn signature(&self) -> &str {
        &self.signature
    }
    fn tokenize(&self, line: &str) -> Vec<String> {
        tokenize_13a_cache(line.to_string())
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
        assert_eq!(res, vec!["Hello", ",", "World", "!"]);

        line = "/usr/sbin/sendmail - 0 errors, 12 warnings";
        res = tokenizer_regex.tokenize(line);
        assert_eq!(
            res,
            vec![
                "/", "usr", "/", "sbin", "/", "sendmail", "-", "0", "errors", ",", "12", "warnings"
            ]
        )
    }

    #[test]
    fn test_tokenize_13a_regex() {
        let tokenizer_regex = tokenizer::Tokenizer13a::new();
        let mut line = "Hello, &quot;World!<skipped>";
        let mut res = tokenizer_regex.tokenize(line);
        assert_eq!(res, vec!["Hello", ",", "\"", "World", "!"]);

        line = "/usr/sbin/sendmail - 0 errors, 12 warnings";
        res = tokenizer_regex.tokenize(line);
        assert_eq!(
            res,
            vec![
                "/", "usr", "/", "sbin", "/", "sendmail", "-", "0", "errors", ",", "12", "warnings"
            ]
        )
    }
}

#[cfg(test)]
mod benchmark {
    use crate::tokenizer;
    use crate::tokenizer::Tokenizer;
    use test::Bencher;
    #[bench]
    fn bench_tokenizer(b: &mut Bencher) {
        let tokenizer_regex = tokenizer::Tokenizer13a::new();
        let line = "Hello, &quot;World!<skipped>";

        let iter_num: usize = 100;
        b.iter(|| {
            std::hint::black_box(for _ in 1..=iter_num {
                tokenizer_regex.tokenize(line);
            });
        });
    }
}
