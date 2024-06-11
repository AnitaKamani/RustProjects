use std::io;
fn main() {
    let mut fibonacci_vec: Vec<i32> = Vec::new();
    fibonacci_vec.push(1);
    fibonacci_vec.push(2);
    for i in 2..11 {
        fibonacci_vec.push(fibonacci_vec[i - 1] + fibonacci_vec[i - 2]);
    }

    let mut input: String = String::new();
    let _ = io::stdin().read_line(&mut input).expect("error");
    let fibonacci_lenth: i32 = input.trim().parse::<i32>().unwrap();

    for _i in 1..fibonacci_lenth + 1 {
        if fibonacci_vec.contains(&_i) {
            print!("+")
        } else {
            print!("-")
        }
    }
    // println!("{:?}", fibonacci_vec);
}
