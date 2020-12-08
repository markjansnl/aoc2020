use aoc05::{input, *};

fn main() {
    let mut seat_ids: Vec<u16> = input::USER
        .lines()
        .map(|boarding_pass| seat_id(boarding_pass))
        .collect();

    seat_ids.sort();

    let min = seat_ids[0];
    let max = seat_ids[seat_ids.len() - 1];

    let missing = seat_ids
        .iter()
        .zip(min..=max)
        .filter(|(a, b)| *a != b)
        .nth(0)
        .unwrap()
        .1;

    println!("{}", missing);
}
