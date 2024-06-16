use std::io;

fn main() {
    let mut input_text = String::new();
    let _ = io::stdin().read_line(&mut input_text).expect("wrong input");

    let trimmed = input_text.trim();
    let mut my_integer = trimmed.parse().unwrap();
    my_integer -= 1;

    let mut i = 0;
    while i <= my_integer {
        let mut j = 0;
        let mut l = "".to_string();
        while j <= my_integer {
            if (i == 0) | (i == my_integer) | (j == 0) | (j == my_integer) {
                l = l + "*";
            } else {
                l = l + " ";
            }

            j += 1;
        }
        println!("{l}");
        j = 0;
        i += 1;
    }
}
