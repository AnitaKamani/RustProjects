use std::io;

fn main() {
    // the number of original chess
    // pieces /1-King/1-Queen/2-rooks/2-Bishops/2-Knights/8-Pawns

    let original_pieces: Vec<i32> = [1, 1, 2, 2, 2, 8].to_vec();
    let mut input = String::new();
    _ = io::stdin().read_line(&mut input).expect("worong input");

    // the number of the pieces that we have
    let in_hand_pieces: Vec<i32> = input
        .trim()
        .split(" ")
        .map(|x| x.parse::<i32>().expect("wrong input"))
        .collect();

    // the number of pieces that we must trade in order to correct the game
    let subtraction: Vec<String> = original_pieces
        .iter()
        .zip(in_hand_pieces)
        .map(|(a, b)| (a - b).to_string())
        .collect();

    println!("{}", subtraction.join(" "));
}
