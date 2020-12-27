use aoc19::{input, Rules};

fn count_valid(input: &str) -> usize {
    let mut input_split = input.split("\n\n");
    let rules: Rules = input_split.next().unwrap().into();

    input_split
        .next()
        .unwrap()
        .lines()
        .filter(|line| {
            rules
                .validate_line(line, 0, 0)
                .iter()
                .any(|len| *len == line.len())
        })
        .count()
}

fn main() {
    println!("{}", count_valid(input::USER));
}

#[test]
fn test_part1_example() {
    assert_eq!(count_valid(input::EXAMPLE), 2);
}
