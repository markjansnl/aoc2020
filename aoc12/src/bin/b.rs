use aoc12::{input, *};

fn main() {
    println!("{}", run(input::USER));
}

fn run(input: &str) -> isize {
    let mut waypoint = Waypoint::new();
    for instruction in parse_input(input) {
        waypoint.apply(&instruction);
    }
    waypoint.manhatten_distance()
}

#[test]
fn test_example() {
    assert_eq!(run(input::EXAMPLE), 286);
}
