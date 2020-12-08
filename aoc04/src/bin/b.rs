use aoc04::{input, *};

fn main() {
    println!("{}", count_valid(input::USER));
}

fn count_valid(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|passport| passport.is_valid_b())
        .count()
}

#[test]
fn test_invalid() {
    assert_eq!(parse_input(input::INVALID).len(), 4);
    assert_eq!(count_valid(input::INVALID), 0);
}

#[test]
fn test_valid() {
    assert_eq!(count_valid(input::VALID), 4);
}
