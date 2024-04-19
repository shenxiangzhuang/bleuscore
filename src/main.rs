fn main() {
    for i in 1..=4 {
        for j in 0..=(2 - i) {
            println!("{i}, {j}");
        }
    }
}