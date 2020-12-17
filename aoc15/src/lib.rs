pub fn spoken_number(starting_numbers: Vec<usize>, nth: usize) -> usize {
    let mut map = vec![0usize; nth];
    for (turn, &starting_number) in starting_numbers[0..starting_numbers.len() - 1].iter().enumerate() {
        map[starting_number] = turn + 1;
    }

    let mut number = starting_numbers[starting_numbers.len() - 1];
    for turn in starting_numbers.len()..nth {
        number = turn - match std::mem::replace(&mut map[number], turn) {
            0               => turn,
            old_value => old_value,
        }
    }

    number
}