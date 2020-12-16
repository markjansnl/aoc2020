use aoc15::*;

fn main() {
    println!("{}", spoken_number(vec![19, 0, 5, 1, 10, 13], 2020));
}

#[test]
fn test_examples() {
    assert_eq!(spoken_number(vec![0, 3, 6], 10), 0);
    assert_eq!(spoken_number(vec![0, 3, 6], 2020), 436);

    assert_eq!(spoken_number(vec![1, 3, 2], 2020), 1);
    assert_eq!(spoken_number(vec![2, 1, 3], 2020), 10);
    assert_eq!(spoken_number(vec![1, 2, 3], 2020), 27);
    assert_eq!(spoken_number(vec![2, 3, 1], 2020), 78);
    assert_eq!(spoken_number(vec![3, 2, 1], 2020), 438);
    assert_eq!(spoken_number(vec![3, 1, 2], 2020), 1836);
}
