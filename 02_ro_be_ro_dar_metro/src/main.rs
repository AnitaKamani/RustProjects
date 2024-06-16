use std::io;

fn main() {
    let mut input_1 = String::new();
    let mut input_2 = String::new();
    _ = io::stdin().read_line(&mut input_1).expect("wrong answer");
    _ = io::stdin().read_line(&mut input_2).expect("wrong answer");

    let input_1_vec: Vec<usize> = input_1
        .trim()
        .split(" ")
        .map(|x| x.parse().expect("Not an intiger"))
        .collect();
    let input_2_vec: Vec<usize> = input_2
        .trim()
        .split(" ")
        .map(|x| x.parse().expect("Not an integer"))
        .collect();

    let ro_be_ro: usize = input_1_vec
        .iter()
        .zip(input_2_vec.iter())
        .map(|(x, y)| x * y)
        .sum();

    println!("{}", ro_be_ro);
}
