use aoc04::{input, *};

fn main() {
    println!("{}", count_valid(input::USER));
}

fn count_valid(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|passport| passport.is_valid_a())
        .count()
}

#[test]
fn test_example() {
    assert_eq!(count_valid(input::EXAMPLE), 2);
}
