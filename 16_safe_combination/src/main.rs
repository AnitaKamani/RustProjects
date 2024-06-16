use std::io;
fn main() {
    let mut input1 = String::new();
    let _ = io::stdin()
        .read_line(&mut input1)
        .expect("error reading input");
    let length = input1.trim().parse::<usize>().unwrap();

    let mut correct_password = String::new();
    let _2 = io::stdin()
        .read_line(&mut correct_password)
        .expect("error reading input");

    let mut input_k = String::new();
    let mut sum = 0;
    for i in 0..length {
        input_k.clear();
        let _3 = io::stdin().read_line(&mut input_k).expect("wrong format");
        if let Some(mut index) = input_k.find(correct_password.chars().nth(i).unwrap()) {
            if 9 - index < index {
                index = 9 - index;
            }

            sum += index;
        } else {
        }
    }

    println!("{sum}");
}
