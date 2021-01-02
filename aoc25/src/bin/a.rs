use aoc25::input;

fn find_encryption_key(input: &str) -> usize {
    let public_keys: Vec<usize> = input
        .lines()
        .map(|public_key| public_key.parse().unwrap())
        .collect();

    let mut accumulator = 1usize;
    let mut loop_size = 0;
    let mut subject_number = 7;
    while accumulator != public_keys[0] && accumulator != public_keys[1] {
        accumulator = (accumulator * subject_number) % 20201227;
        loop_size += 1;
    }

    if accumulator == public_keys[0] {
        subject_number = public_keys[1];
    } else {
        subject_number = public_keys[0];
    }
    accumulator = 1;
    for _ in 0..loop_size {
        accumulator = (accumulator * subject_number) % 20201227;
    }

    accumulator
}

fn main() {
    println!("{}", find_encryption_key(input::USER));
}

#[test]
fn test_part1_example() {
    assert_eq!(find_encryption_key(input::EXAMPLE), 14897079);
}
