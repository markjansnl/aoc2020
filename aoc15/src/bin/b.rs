use aoc15::*;

fn main() {
    println!("{}", spoken_number(vec![19, 0, 5, 1, 10, 13], 30_000_000));
}

#[test]
fn test_examples() {
    assert_eq!(spoken_number(vec![0, 3, 6], 30_000_000), 175594);

    assert_eq!(spoken_number(vec![1, 3, 2], 30_000_000), 2578);
    assert_eq!(spoken_number(vec![2, 1, 3], 30_000_000), 3544142);
    assert_eq!(spoken_number(vec![1, 2, 3], 30_000_000), 261214);
    assert_eq!(spoken_number(vec![2, 3, 1], 30_000_000), 6895259);
    assert_eq!(spoken_number(vec![3, 2, 1], 30_000_000), 18);
    assert_eq!(spoken_number(vec![3, 1, 2], 30_000_000), 362);
}
