use counter::Counter;

fn main() {
    let mut counts = "able babble table babble rabble table able fable scrabble"
        .split_whitespace().collect::<Counter<_>>();
}