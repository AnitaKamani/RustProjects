use std::io;

fn main() {
    let mut input: String = String::new();
    _ = io::stdin().read_line(&mut input).expect("Wrong input");
    let mazroub: usize = input.trim().parse::<usize>().unwrap();
    for i in 1..mazroub + 1 {
        for j in 1..mazroub + 1 {
            print!("{} ", i * j);
        }
        println!("");
    }
}
