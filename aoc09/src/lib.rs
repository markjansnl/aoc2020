pub mod input;
use std::collections::{BTreeSet, VecDeque};

pub fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn find_encryption_weakness(
    numbers: &Vec<usize>,
    preamble_len: usize,
) -> Option<(usize, usize)> {
    let first_wrong_number = find_first_wrong_number(numbers, preamble_len).unwrap();
    let mut encryption_weakness = VecDeque::new();
    let mut sum = 0;

    for number in numbers {
        encryption_weakness.push_back(*number);
        sum += *number;

        while sum > first_wrong_number {
            sum -= encryption_weakness.pop_front().unwrap();
        }

        if sum == first_wrong_number {
            return Some(
                encryption_weakness
                    .iter()
                    .fold((usize::MAX, usize::MIN), |(min, max), number| {
                        (min.min(*number), max.max(*number))
                    }),
            );
        }
    }

    None
}

pub fn find_first_wrong_number(numbers: &Vec<usize>, preamble_len: usize) -> Option<usize> {
    let mut deque = VecDeque::with_capacity(preamble_len);
    let mut set = BTreeSet::new();

    for (index, number) in numbers.iter().enumerate() {
        if index >= preamble_len {
            if is_valid(&set, *number).is_none() {
                return Some(*number);
            }
            let remove_number = deque.pop_front().unwrap();
            set.remove(&remove_number);
        }
        deque.push_back(*number);
        set.insert(*number);
    }

    None
}

fn is_valid(set: &BTreeSet<usize>, number: usize) -> Option<(usize, usize)> {
    for (first, second) in set
        .iter()
        .filter(|first| **first < number)
        .map(|first| (first, number - first))
    {
        if *first != second && set.contains(&second) {
            return Some((*first, second));
        }
    }
    None
}
