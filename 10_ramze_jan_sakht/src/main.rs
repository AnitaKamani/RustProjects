use std::io;

fn check_aval(c: usize) -> bool {
    if c == 0 {
        return true;
    } else if c == 1 {
        return false;
    } else if c <= 2 {
        return true;
    }
    for i in 2..((c as f64).sqrt() as usize) + 1 {
        if c % i == 0 {
            return false;
        }
    }
    check_aval(c / 10)
}

fn main() {
    let mut input: String = String::new();
    let _ = io::stdin().read_line(&mut input).expect("Wrong input");
    let number = input.trim().parse::<u32>().unwrap();
    let mut sets: Vec<Vec<usize>> = Vec::new();

    sets.push(vec![2, 3, 5, 7]);
    for _ in 1..number {
        sets.push(vec![1, 3, 5, 7, 9]);
    }
    let mut results = vec![];
    generate_combinations(&sets, 0, 0, &mut results, number);

    for result in results {
        if check_aval(result) {
            println!("{}", result);
        }
    }
}

fn generate_combinations(
    sets: &Vec<Vec<usize>>,
    depth: usize,
    current_sum: usize,
    results: &mut Vec<usize>,
    number: u32,
) {
    if depth == sets.len() {
        results.push(current_sum);
        return;
    }

    for &value in &sets[depth] {
        if current_sum == 0 || check_aval(current_sum) {
            generate_combinations(sets, depth + 1, current_sum * 10 + value, results, number);
        }
    }
}
