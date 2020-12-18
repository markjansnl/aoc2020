use aoc16::{input, Input};

fn main() {
    println!("{}", Input::from(input::USER).ticket_scanning_error_rate());
}

#[test]
fn test_example() {
    assert_eq!(Input::from(input::EXAMPLE).ticket_scanning_error_rate(), 71);
}
