use aoc10::input;
use std::collections::HashMap;

fn main() {
    println!("{}", adapter_arrangements(input::USER));
}

fn adapter_arrangements(input: &str) -> usize {
    let mut adapters: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
    adapters.push(0);
    adapters.sort();

    let device_adapter = adapters[adapters.len() - 1] + 3;
    adapters.push(device_adapter);
    adapters.push(usize::MAX);
    adapters.push(usize::MAX);
    adapters.reverse();

    let mut branches: HashMap<usize, usize> = HashMap::new();
    branches.insert(device_adapter, 1);
    branches.insert(usize::MAX, 0);

    adapters.windows(4).for_each(|window| {
        branches.insert(
            window[3],
            (0usize..=2)
                .into_iter()
                .filter(|i| window[*i] <= window[3] + 3)
                .map(|i| branches[&window[i]])
                .sum()
        );
    });

    branches[&0]
}

#[test]
fn test_example1() {
    assert_eq!(adapter_arrangements(input::EXAMPLE1), 8);
}

#[test]
fn test_example2() {
    assert_eq!(adapter_arrangements(input::EXAMPLE2), 19208);
}
