use aoc06::input;
use std::collections::HashMap;

fn main() {
    println!("{}", sum_counts(input::USER));
}

fn sum_counts(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let people_in_group = group.lines().count();
            group
                .chars()
                .filter(|c| !c.is_whitespace())
                .fold(HashMap::new(), |mut map, c| {
                    map.entry(c)
                        .and_modify(|char_count| *char_count += 1)
                        .or_insert(1usize);
                    map
                })
                .values()
                .filter(|char_count| **char_count == people_in_group)
                .count()
        })
        .sum()
}

#[test]
fn test_example() {
    assert_eq!(sum_counts(input::EXAMPLE), 6);
}
