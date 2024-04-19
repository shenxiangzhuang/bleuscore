use counter::Counter;


pub fn get_ngram_counter(line: &str, max_order: usize) -> Counter<&str> {
    let mut counts: Counter<&str> = Counter::new();
    for order in 1..=max_order {
        for start_index in 0..=(line.len() - order) {
            if (start_index + order) <= line.len() {
                // println!("line: {}, start_index: {}, order: {}", line, start_index, order);
                let ngram = &line[start_index..(start_index + order)];
                // println!("ngram: {}", ngram);
                counts[&ngram] += 1;
            }
            // why have to break here?(If no break, will hang here)
            else{
                break
            }
        }
    }
    counts
}


#[cfg(test)]
mod test {
    use crate::ngram::{get_ngram_counter};

    #[test]
    fn test_get_ngram() {
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