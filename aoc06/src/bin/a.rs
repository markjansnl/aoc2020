use aoc06::input;

fn main() {
    println!("{}", sum_counts(input::USER));
}

fn sum_counts(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let mut vec: Vec<char> = group.chars().filter(|c| !c.is_whitespace()).collect();
            vec.sort();
            vec.dedup();
            vec.len()
        })
        .sum()
}

#[test]
fn test_example() {
    assert_eq!(sum_counts(input::EXAMPLE), 11);
}
