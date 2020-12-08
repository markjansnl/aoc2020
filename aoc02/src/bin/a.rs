use aoc02::{*, input};

fn main() {
    println!("{}", count_valid_a(input::USER));
}

#[test]
fn test_example()  {
    assert_eq!(count_valid_a(input::EXAMPLE), 2);
}
