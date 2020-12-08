use aoc03::{input, *};

fn main() {
    let map: Map = input::USER.into();

    println!("{}", check_slopes(&map));
}

fn check_slopes(map: &Map) -> usize {
    // Iterators seem overkill here, but when slopes would be very long,
    // mapping could be done in parallel in a thread pool with rayon

    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(right, down)| map.count_trees(*right, *down))
        .product()
}

#[test]
fn test_example() {
    let map: Map = input::EXAMPLE.into();

    assert_eq!(check_slopes(&map), 336);
}
