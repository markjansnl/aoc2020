use aoc05::{input, *};

fn main() {
    let highest_seat_id = input::USER
        .lines()
        .map(|boarding_pass| seat_id(boarding_pass))
        .max()
        .unwrap();

    println!("{}", highest_seat_id);
}
