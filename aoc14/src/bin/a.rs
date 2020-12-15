use aoc14::input;
use std::collections::HashMap;

#[derive(Debug)]
enum Command {
    SetBitmask { or: u64, and: u64 },
    SetValue { address: u64, value: u64 },
}

fn parse_input(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(&[' ', '=', '[', ']'][..]).collect();
            match split[0] {
                "mask" => {
                    let (or, and) = split[3].bytes().fold((0, u64::MAX), |(or, and), byte| {
                        (
                            (or << 1) + if byte == b'1' { 1 } else { 0 },
                            (and << 1) + if byte == b'0' { 0 } else { 1 },
                        )
                    });
                    Command::SetBitmask { or, and }
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
struct Memory {
    bitmask_or: u64,
    bitmask_and: u64,
    map: HashMap<u64, u64>,
}

impl Memory {
    pub fn apply(&mut self, command: Command) {
        match command {
            Command::SetBitmask { or, and } => {
                self.bitmask_or = or;
                self.bitmask_and = and;
            }
            Command::SetValue { address, value } => {
                self.map
                    .insert(address, (value | self.bitmask_or) & self.bitmask_and);
            }
        }
    }

    pub fn sum_values(&self) -> u64 {
        self.map.iter().fold(0, |acc, (_, value)| acc + value)
    }
}

fn sum_memory(input: &str) -> u64 {
    let mut memory = Memory::default();
    for command in parse_input(input) {
        memory.apply(command);
    }
    memory.sum_values()
}

fn main() {
    println!("{}", sum_memory(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(sum_memory(input::EXAMPLE), 165);
}
