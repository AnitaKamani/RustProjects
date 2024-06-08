use std::io;
fn main() {
    let mut input_text = String::new();
    let _ = io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read line");

    // convert to int to remove the leading 0s
    let my_int = input_text.trim().parse::<i32>().unwrap();
    // convert to string and the to chars
    let char_array: Vec<char> = my_int.to_string().chars().collect();

    // print!("{:?}", char_array);
    let l = char_array.len() as usize;
    let h = ((l - 1) / 2) as usize;

    let mut khod_maghloob = "YES";
    for i in 0..h {
        if char_array[i] != char_array[l - i - 1] {
            khod_maghloob = "NO";
        }
    }
    println!("{}", khod_maghloob)
}
