use std::io;

fn get_divisors(number: usize) -> Vec<usize> {
    let mut divisors_list: Vec<usize> = Vec::new();
    for i in 1..number {
        if number % i == 0 {
            divisors_list.push(i);
        }
    }
    return divisors_list;
}
fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input).expect("Wrong input");
    let number: usize = input.trim().parse::<usize>().unwrap();
    let divisors: Vec<usize> = get_divisors(number);
    let sum: usize = divisors.iter().sum();
    if sum == number {
        println!("YES");
    } else {
        println!("NO");
    }
}
