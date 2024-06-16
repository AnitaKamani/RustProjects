use std::io;
fn main() {
    //  get input
    let mut string1 = String::new();
    let mut string2 = String::new();
    let _ = io::stdin().read_line(&mut string1).expect("wrong input");
    let _2 = io::stdin().read_line(&mut string2).expect("wrong input");
    // string input to integer number
    let mut vec1: Vec<usize> = string1
        .trim()
        .split(" ")
        .map(|x| x.parse::<usize>().expect("wrong input"))
        .collect();
    let mut vec2: Vec<usize> = string2
        .trim()
        .split(" ")
        .map(|x| x.parse::<usize>().expect("wrong input"))
        .collect();

    let mut escape = false;
    'outer: for _i in 0..4 {
        for _j in 0..4 {
            let a = ((vec1[1] + vec2[1]) % 10) * 100;
            let b = ((vec1[2] + vec2[2]) % 10) * 10;
            let c = (vec1[3] + vec2[3]) % 10;
            let d = (a + b + c) % 6;

            if d == 0 {
                // println!("{:?}", vec1);
                // println!("{:?}", vec2);
                println!("Boro joloo :)");
                escape = true;
                break 'outer;
            }
            // rotate the  inner vector
            let inter2 = vec2.pop().unwrap();
            vec2.insert(0, inter2);
        }
        // rotate the  outer vector
        let inter = vec1.pop().unwrap();
        vec1.insert(0, inter);
    }
    if !escape {
        println!("Gir oftadi :(")
    }
}
