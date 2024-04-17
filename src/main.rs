use uluru::LRUCache;

#[derive(Debug)]
pub struct StringTokens {
    string: String,
    tokens: Vec<String>
}

type StringTokensCache = LRUCache<StringTokens, 65534>;

fn main() {
    let mut cache = StringTokensCache::default();
    cache.insert(StringTokens{string: "Hello".to_string(), tokens: vec!["H".to_string(), "O".to_string()]});
    let res = cache.find(|x| x.string == "Hello").unwrap().tokens.clone();
    println!("{:?}", res);
}