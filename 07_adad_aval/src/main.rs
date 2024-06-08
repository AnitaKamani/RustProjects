use std::io;

fn aval_numbers(a: usize, b: usize) -> Vec<usize> {
    let mut aval_nums: Vec<usize> = Vec::new();

    for i in a..b + 1 {
        let mut is_aval: bool = true;
        // println!("{}", i);
        for j in 1..i {
            if i % j == 0 && j != 1 {
                // println!("{}, {}", i, j);
                is_aval = false;
            }
        }
        if is_aval && i != 1 {
            aval_nums.push(i);
        }
    }
    return aval_nums;
}

fn main() {
    let mut input: String = String::new();
    _ = io::stdin().read_line(&mut input).expect("wrong input");
    let a: usize = input.trim().parse::<usize>().unwrap();

    let mut input2: String = String::new();
    _ = io::stdin().read_line(&mut input2).expect("wrong input");
    let b: usize = input2.trim().parse::<usize>().unwrap();
    let number_list: Vec<usize> = aval_numbers(a, b);

    for i in number_list {
        println!("{}", i);
    }
}
