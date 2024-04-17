fn main() {
    let line = "Hello  ,     World !";
    println!("{:?}", line.split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>());
}