use std::collections::{HashMap, HashSet};

use aoc14::input;

#[derive(Debug)]
enum Command {
    SetBitmask { one: u64, floating: u64 },
    SetValue { address: u64, value: u64 },
}

fn parse_input(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(&[' ', '=', '[', ']'][..]).collect();
            match split[0] {
                "mask" => {
                    let (one, floating) = split[3].bytes().fold((0, 0), |(one, floating), byte| {
                        (
                            (one << 1) + if byte == b'1' { 1 } else { 0 },
                            (floating << 1) + if byte == b'X' { 1 } else { 0 },
                        )
                    });
                    Command::SetBitmask { one, floating }
                }
                "mem" => Command::SetValue {
                    address: split[1].parse().unwrap(),
                    value: split[5].parse().unwrap(),
                },
                _ => panic!("Invalid input"),
            }
        })
        .collect()
}
#[derive(Default, Debug)]
struct Range {
    bitmask_floating: u64,
    value: u64,
    excluded_bitmasks: HashSet<u64>,
    count: u32,
}

#[derive(Default, Debug)]
struct Memory {
    bitmask_one: u64,
    bitmask_floating: u64,
    ranges: HashMap<u64, Range>,
}

impl Memory {
    pub fn apply(&mut self, command: Command) {
        match command {
            Command::SetBitmask { one, floating } => {
                self.bitmask_one = one;
                self.bitmask_floating = floating;
            }
            Command::SetValue { address, value } => {
                let base_address = (address | self.bitmask_one) & !self.bitmask_floating;

                for (base, range) in self.ranges.iter_mut() {
                    if base & !self.bitmask_floating == base_address & !range.bitmask_floating {
                        let mut do_nothing = false;
                        
                        for excluded_bitmask in range.excluded_bitmasks.clone().into_iter() {
                            if self.bitmask_floating & !excluded_bitmask == 0 {
                                // Same or larger one is already excluded
                                do_nothing = true;
                                break;
                            } else if excluded_bitmask & !self.bitmask_floating == 0 {
                                // A smaller one is already there, remove it
                                range.excluded_bitmasks.remove(&excluded_bitmask);
                                range.count += 2u32.pow(excluded_bitmask.count_ones());
                                break;
                            }
                        }
                    
                        if !do_nothing {
                            range.excluded_bitmasks.insert(self.bitmask_floating);
                            let sub = 2u32.pow(range.bitmask_floating.count_ones());
                            if sub > range.count {
                                // println!("{:#?}, {}", range, sub);
                            } else {
                                range.count -= sub;
                            }
                            println!("{:#?}", range);
                        }
                        // println!("Old: {:#?}", range);
                        // range.bitmask_floating = range.bitmask_floating & !self.bitmask_floating;
                        // println!("New: {:?}", range);
                    }
                }

                self.ranges.insert(
                    base_address,
                    Range {
                        bitmask_floating: self.bitmask_floating,
                        value,
                        excluded_bitmasks: HashSet::new(),
                        count: 2u32.pow(self.bitmask_floating.count_ones()),
                    },
                );
            }
        }
    }

    pub fn sum_values(&self) -> u32 {
        self.ranges.iter().fold(0, |acc, (_, range)| acc + range.count)
    }

    pub fn print_excluded_ranges(&self) {
        for (_, range) in self.ranges.iter() {
            if !range.excluded_bitmasks.is_empty() {
                println!("{:#?}", range);
            }
        }
    }
}

fn sum_memory(input: &str) -> u32 {
    let mut memory = Memory::default();
    for command in parse_input(input) {
        // println!("{:?}", &command);
        memory.apply(command);
    }
    memory.print_excluded_ranges();
    memory.sum_values()
}

fn main() {
    println!("{}", sum_memory(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(sum_memory(input::EXAMPLE2), 208);
}


// Too low:  5140366174772
// Too high: 5170037444560