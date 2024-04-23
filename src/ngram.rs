use std::collections::HashMap;
use counter::Counter;


/// Here the tokens' type is `&[String]` rather than `&Vec<String>`
/// to fix `clippy::not_unsafe_ptr_arg_deref` error.
pub fn get_token_ngram_counter(tokens: &[String], 
                               max_order: usize,
                              ) -> HashMap<(String, usize), usize> 
{
    let mut count_map: HashMap<(String, usize), usize> = HashMap::new();
    for order in 1..=max_order {
        for start_index in 0..(tokens.len().saturating_sub(order - 1)) {
            // note: can not join with "", which will make 2-gram ('000', '00') = ('0000', '0')
            let ngram = tokens[start_index..(start_index + order)].join(" ");
            count_map.entry((ngram, order))
                     .and_modify(|counter| *counter += 1)
                     .or_insert(1);
        }
    }
    count_map
}


/// TODO: change to use Counter to count ngram
#[allow(dead_code)]
fn get_ngram_counter(line: &str, max_order: usize) -> Counter<&str> {
    let mut counts: Counter<&str> = Counter::new();
    for order in 1..=max_order {
        for start_index in 0..(line.len().saturating_sub(order - 1)) {
            // println!("line: {}, start_index: {}, order: {}", line, start_index, order);
            let ngram = &line[start_index..(start_index + order)];
            // println!("ngram: {}", ngram);
            counts[&ngram] += 1;
        }
    }
    counts
}


#[cfg(test)]
mod test {
    use crate::ngram::{get_ngram_counter, get_token_ngram_counter};

    #[test]
    fn test_get_token_ngram_short() {
        let tokens = vec!["a".to_string(), "b".to_string()];
        let counter = get_token_ngram_counter(&tokens,4);
        assert_eq!(counter[&("a".to_string(), 1)], 1);
        assert_eq!(counter[&("b".to_string(), 1)], 1);
        assert_eq!(counter[&("a b".to_string(), 2)], 1);
    }

    #[test]
    fn test_get_token_ngram_long() {
        // aabc
        let tokens: Vec<String> = vec!["a".to_string(),
                                       "a".to_string(),
                                       "b".to_string(),
                                       "c".to_string()];
        let counter = get_token_ngram_counter(&tokens,4);
        assert_eq!(counter[&("a".to_string(), 1)], 2);
        assert_eq!(counter[&("b".to_string(), 1)], 1);
        assert_eq!(counter[&("c".to_string(), 1)], 1);
        assert_eq!(counter.get(&("d".to_string(), 1)), None);

        assert_eq!(counter[&("a a".to_string(), 2)], 1);
        assert_eq!(counter[&("a b".to_string(), 2)], 1);
        assert_eq!(counter[&("b c".to_string(), 2)], 1);
        assert_eq!(counter.get(&("a c".to_string(), 2)), None);

        assert_eq!(counter[&("a a b".to_string(), 3)], 1);
        assert_eq!(counter[&("a b c".to_string(), 3)], 1);
        assert_eq!(counter[&("a a b c".to_string(), 4)], 1);

        assert_eq!(counter.len(), 9);
    }


    #[test]
    fn test_get_ngram_short() {
        let counter = get_ngram_counter("ab", 4);
        assert_eq!(counter[&"a"], 1);
        assert_eq!(counter[&"b"], 1);
        assert_eq!(counter[&"ab"], 1);
    }

    #[test]
    fn test_get_ngram_long() {
        let counter = get_ngram_counter("aabc", 4);
        assert_eq!(counter[&"a"], 2);
        assert_eq!(counter[&"b"], 1);
        assert_eq!(counter[&"c"], 1);
        assert_eq!(counter[&"d"], 0);

        assert_eq!(counter[&"aa"], 1);
        assert_eq!(counter[&"ab"], 1);
        assert_eq!(counter[&"bc"], 1);
        assert_eq!(counter[&"ac"], 0);

        assert_eq!(counter[&"aab"], 1);
        assert_eq!(counter[&"aabc"], 1);
    }
}