use std::{collections::HashMap, vec};

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
struct Memory {
    bitmask_one: u64,
    bitmask_floating: u64,
    map: HashMap<u64, u64>,
}

impl Memory {
    pub fn apply(&mut self, command: Command) {
        match command {
            Command::SetBitmask { one, floating } => {
                self.bitmask_one = one;
                self.bitmask_floating = floating;
            }
            Command::SetValue { address, value } => {
                for addr in get_addresses(vec![address | self.bitmask_one], self.bitmask_floating) {
                    self.map.insert(addr, value);
                }
            }
        }
    }

    pub fn sum_values(&self) -> u64 {
        self.map.iter().fold(0, |acc, (_, value)| acc + value)
    }
}

fn get_addresses(from: Vec<u64>, bitmask: u64) -> Vec<u64> {
    // Make sure the recursive function stops
    if bitmask == 0 {
        return from;
    }

    // Find most right X
    let mut addresses = vec![];
    let mut one_bitmask = 1;
    while one_bitmask & !bitmask > 0 {
        one_bitmask <<= 1;
    }

    // Replace the X for all items in from with 0 and 1
    for address in from {
        addresses.push(address & !one_bitmask);
        addresses.push(address | one_bitmask);
    }

    // Continue till we have all X'es replaced
    get_addresses(addresses, bitmask & !one_bitmask)
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
    assert_eq!(sum_memory(input::EXAMPLE2), 208);
}

#[test]
fn test_addresses() {
    assert_eq!(get_addresses(vec![0], 1), vec![0, 1]);
    assert_eq!(get_addresses(vec![0], 6), vec![0, 4, 2, 6]);
    assert_eq!(get_addresses(vec![2], 5), vec![2, 6, 3, 7]);
}
