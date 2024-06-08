use std::io;
fn main() {
    let mut input = String::new();
    let _ = io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    // println!("{}", input);
    let naseza = input.trim().parse::<usize>().unwrap();
    if ((naseza / 2) as usize) * 2 == naseza {
        println!("Bala Barare");
    } else {
        println!("Payin Barare");
    }
}
