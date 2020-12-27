use aoc19::{Rule, Rules, input};

fn count_valid(input: &str) -> usize {
    let mut input_split = input.split("\n\n");
    let mut rules: Rules = input_split.next().unwrap().into();

    rules.insert(8, Rule::SubRules(vec![vec![42], vec![42, 8]]));
    rules.insert(11, Rule::SubRules(vec![vec![42, 31], vec![42, 11, 31]]));

    input_split
        .next()
        .unwrap()
        .lines()
        .filter_map(|line| {
            rules
                .validate_line(line, 0)
                .filter(|len| *len == line.len())
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
