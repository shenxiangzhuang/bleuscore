use ahash::AHashMap;

/// Here the tokens' type is `&[String]` rather than `&Vec<String>`
/// to fix `clippy::not_unsafe_ptr_arg_deref` error.
pub fn get_token_ngram_counter(tokens: &[String], max_order: usize) -> AHashMap<&[String], usize> {
    let mut count_map: AHashMap<&[String], usize> = AHashMap::new();
    for order in 1..=max_order {
        for start_index in 0..tokens.len().saturating_sub(order - 1) {
            let ngram = &tokens[start_index..(start_index + order)];
            count_map
                .entry(ngram)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
    }
    count_map
}

#[cfg(test)]
mod test {
    use crate::ngram::get_token_ngram_counter;

    #[test]
    fn test_get_token_ngram_short() {
        let tokens = vec!["a".to_string(), "b".to_string()];
        let counter = get_token_ngram_counter(&tokens, 4);
        assert_eq!(counter[&tokens[0..=0]], 1);
        assert_eq!(counter[&tokens[1..=1]], 1);
        assert_eq!(counter[&tokens[0..=1]], 1);
    }

    #[test]
    fn test_get_token_ngram_long() {
        // aabc
        let tokens: Vec<String> = vec![
            "a".to_string(),
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
        ];
        let counter = get_token_ngram_counter(&tokens, 4);
        assert_eq!(counter[&tokens[0..=0]], 2); // 'a': 2
        assert_eq!(counter[&tokens[2..=2]], 1); // 'b': 1
        assert_eq!(counter[&tokens[3..=3]], 1); // 'c': 1

        assert_eq!(counter[&tokens[0..=1]], 1); // 'aa': 1
        assert_eq!(counter[&tokens[1..=2]], 1); // 'ab': 1
        assert_eq!(counter[&tokens[2..=3]], 1); // 'bc': 1

        assert_eq!(counter[&tokens[0..=2]], 1); // 'aab': 1
        assert_eq!(counter[&tokens[1..=3]], 1); // 'abc': 1
        assert_eq!(counter[&tokens[0..=3]], 1); // 'abcd': 1

        assert_eq!(counter.len(), 9);
    }
}

#[cfg(test)]
mod benchmark {
    use crate::ngram::get_token_ngram_counter;
    use test::Bencher;

    #[bench]
    fn bench_ngram(b: &mut Bencher) {
        let tokens: Vec<String> = vec![
            "a".to_string(),
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
        ];
        let max_order = 4;

        let iter_num: usize = 100;
        b.iter(|| {
            std::hint::black_box(for _ in 1..=iter_num {
                get_token_ngram_counter(&tokens, max_order);
            });
        });
    }
}
