use std::io;
fn main() {
    let mut length_string = String::new();
    let _ = io::stdin()
        .read_line(&mut length_string)
        .expect("wrong input");
    let length = length_string.trim().parse::<usize>().unwrap();

    let mut numbers_string = String::new();
    let _2 = io::stdin()
        .read_line(&mut numbers_string)
        .expect("wrong input");

    let mut numbers: Vec<i32> = numbers_string
        .trim()
        .split(" ")
        .map(|x| x.parse::<i32>().expect("not int"))
        .collect();
    numbers.sort();

    let mut ouput: Vec<i32> = Vec::new();
    for i in 0..(length / 2) + 1 {
        ouput.push(numbers[length - i - 1]);
        ouput.push(numbers[i]);
    }

    let output_string: Vec<String> = ouput.iter().map(|x| x.to_string()).collect();

    println!("{}", &output_string[0..length].join(" "));
}
