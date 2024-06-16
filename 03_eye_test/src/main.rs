use std::io;

fn main() {
    let mut char_vec: Vec<Vec<char>> = Vec::new();
    let mut input_text = String::new();
    let _ = io::stdin().read_line(&mut input_text);
    let my_integer: usize = input_text.trim().parse().expect("Not a valid integer");
    for _i in 0..2 {
        let mut input2 = String::new();
        let _ = io::stdin().read_line(&mut input2);
        let input_vec: Vec<char> = input2.trim().chars().collect();
        char_vec.push(input_vec);
    }
    let mut counter = 0;
    for i in 0..my_integer {
        if char_vec[0][i] != char_vec[1][i] {
            counter += 1;
        }
    }
    print!("{}", counter);
}
