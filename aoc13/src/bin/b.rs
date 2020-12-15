use num::integer::*;

use aoc13::{input, *};

fn main() {
    println!("{}", earliest_timestamp(input::USER));
}

fn earliest_timestamp(input: &str) -> usize {
    let schedules = parse_input(input).1;
    let busses: Vec<(usize, usize)> = schedules
        .iter()
        .enumerate()
        .filter_map(|(index, schedule)| match schedule {
            Schedule::Bus(bus_id) => Some((index, *bus_id)),
            _ => None,
        }).collect();

    busses[1..].iter().fold((0, busses[0].1), |(offset1, step1), (offset2, step2)| {
            let mut times1 = 1usize;
            let mut times2 = 0usize;
            loop {
                let diff = (offset1 + offset2 + times1 * step1) as isize - (times2 * step2) as isize;
                match diff.signum() {
                    0 => return (offset1 + times1 * step1, step1.lcm(step2)),
                    -1 => times1 += (diff.abs() as usize).div_ceil(&step1),
                    1 => times2 += (diff.abs() as usize).div_ceil(&step2),
                    _ => unreachable!()
                }
            }
        })
        .0
}

#[test]
fn test_examples() {
    assert_eq!(earliest_timestamp(input::EXAMPLE), 1068781);

    assert_eq!(earliest_timestamp("0\n17,x,13"), 102);
    assert_eq!(earliest_timestamp("0\n17,x,13,19"), 3417);
    assert_eq!(earliest_timestamp("0\n67,7,59,61"), 754018);
    assert_eq!(earliest_timestamp("0\n67,x,7,59,61"), 779210);
    assert_eq!(earliest_timestamp("0\n67,7,x,59,61"), 1261476);
    assert_eq!(earliest_timestamp("0\n1789,37,47,1889"), 1202161486);
}
