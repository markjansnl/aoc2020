use aoc12::{input, *};

fn main() {
    println!("{}", run(input::USER));
}

fn run(input: &str) -> isize {
    let mut ship = Ship::default();
    for instruction in parse_input(input) {
        ship.apply(&instruction);
    }
    ship.manhatten_distance()
}

#[test]
fn test_example() {
    assert_eq!(run(input::EXAMPLE), 25);
}
