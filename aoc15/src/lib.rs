pub fn spoken_number(starting_numbers: Vec<usize>, nth: usize) -> usize {
    let mut map = vec![0usize; nth];
    for turn in 1..starting_numbers.len() {
        map[starting_numbers[turn - 1]] = turn;
    }

    let mut number = starting_numbers[starting_numbers.len() - 1];
    for turn in starting_numbers.len()..nth {
        number = match std::mem::replace(&mut map[number], turn) {
            0               => 0,
            old_value => turn - old_value,
        }
    }

    number
}