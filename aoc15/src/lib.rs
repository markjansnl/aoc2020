use std::collections::HashMap;

pub fn spoken_number(starting_numbers: Vec<usize>, nth: usize) -> usize {
    let mut map = HashMap::new();
    for (turn, &starting_number) in starting_numbers[0..starting_numbers.len() - 1].iter().enumerate() {
        map.insert(starting_number, turn);
    }

    let mut number = starting_numbers[starting_numbers.len() - 1];
    for turn in map.len()..nth - 1 {
        number = turn - map.insert(number, turn).unwrap_or(turn);
    }

    number
}