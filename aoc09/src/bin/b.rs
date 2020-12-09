use aoc09::{find_encryption_weakness, input, parse_input};

fn main() {
    let (smallest, largest) = find_encryption_weakness(&parse_input(input::USER), 25).unwrap();
    println!("{}", smallest + largest);
}

#[test]
fn test_example() {
    assert_eq!(find_encryption_weakness(&parse_input(input::EXAMPLE), 5).unwrap(), (15, 47));
}
