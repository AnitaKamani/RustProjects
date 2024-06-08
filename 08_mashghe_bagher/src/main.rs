use std::io;

fn main() {
    let mut input = String::new();
    _ = io::stdin().read_line(&mut input).expect("wrong answer");
    println!("{}", input);
}
