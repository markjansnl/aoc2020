use aoc09::{find_first_wrong_number, input, parse_input};

fn main() {
    println!("{}", find_first_wrong_number(&parse_input(input::USER), 25).unwrap());
}

#[test]
fn test_example() {
    assert_eq!(find_first_wrong_number(&parse_input(input::EXAMPLE), 5).unwrap(), 127);
}
