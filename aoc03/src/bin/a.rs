use aoc03::{input, *};

fn main() {
    let map: Map = input::USER.into();

    println!("{}", map.count_trees(3, 1));
}

#[test]
fn test_example() {
    let map: Map = input::EXAMPLE.into();

    assert_eq!(map.count_trees(3, 1), 7);
}
