use std::io;

fn main() {
    let mut input = String::new();
    _ = io::stdin().read_line(&mut input).expect("wrong answer");
    let inputs: Vec<usize> = input
        .trim()
        .split(" ")
        .map(|x| x.parse().expect("Not an intiger"))
        .collect();
    let sum: usize = inputs.iter().sum();

    let all_greather_than_zero: bool = inputs.iter().all(|&x| x > 0);
    let sum_check: bool = sum == 180;

    if all_greather_than_zero && sum_check {
        println!("Yes");
    } else {
        println!("No");
    }
}
