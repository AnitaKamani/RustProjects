use std::io;

fn main() {
    let mut input = String::new();
    let mut vec_inputs: Vec<String> = Vec::<String>::new();
    //  read input in lines
    loop {
        input.clear();
        let _ = io::stdin().read_line(&mut input).expect("wrong input");
        if input.trim() == "0" {
            break;
        }
        //  insert the number to the begining of the vectorF
        vec_inputs.insert(0, (input.trim()).to_string());
    }

    for i in vec_inputs {
        println!("{i}")
    }
}
