use std::io;
use std::io::prelude::*;

fn main() {
    let mut v = Vec::new();
    for x in 0..2 {
        let mut input = String::new();
        io::stdin().read_line(&mut input);

        v.push(input);
    }
    // println!("{v}");
}
