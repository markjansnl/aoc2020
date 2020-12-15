use aoc14::{input, *};

fn main() {
    println!("{}", sum_memory(input::USER));
}

fn sum_memory(input: &str) -> u64 {
    let mut memory = Memory::default();
    for command in parse_input(input) {
        memory.apply(command);
    }
    memory.sum_values()
}

#[test]
fn test_example() {
    assert_eq!(sum_memory(input::EXAMPLE), 165);
}
