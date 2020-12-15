use aoc13::{input, *};

fn main() {
    println!("{}", earliest_bus(input::USER));
}

fn earliest_bus(input: &str) -> usize {
    let (timestamp, schedules) = parse_input(input);
    schedules
        .iter()
        .filter_map(|schedule| match schedule {
            Schedule::Bus(bus_id) => Some((bus_id, bus_id - (timestamp % bus_id))),
            _ => None,
        })
        .min_by(|(_, a), (_, b)| a.cmp(&b))
        .map(|(bus_id, duration)| bus_id * duration)
        .unwrap()
}

#[test]
fn test_example() {
    assert_eq!(earliest_bus(input::EXAMPLE), 295);
}
