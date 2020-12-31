use std::collections::HashMap;

use aoc24::input;

fn count_black_tiles(input: &str) -> usize {
    input
        .lines()
        .fold(
            HashMap::new(),
            |mut map: HashMap<(i32, i32), bool>, line| {
                map.entry(
                    line.bytes()
                        .fold((0i32, (0i32, 0i32)), |(dy, (y, x)), byte| match byte {
                            b'n' => (-1, (y, x)),
                            b's' => (1, (y, x)),
                            b'e' => (0, (y + dy, x + 1 - (dy * (y + 1)).abs() % 2)),
                            b'w' => (0, (y + dy, x - 1 + (dy * y).abs() % 2)),
                            _ => panic!("Wrong input"),
                        })
                        .1,
                )
                .and_modify(|tile_is_black| *tile_is_black = !*tile_is_black)
                .or_insert(true);
                map
            },
        )
        .into_iter()
        .filter(|(_, tile_is_black)| *tile_is_black)
        .count()
}

fn main() {
    println!("{}", count_black_tiles(input::USER));
}

#[test]
fn test_part1_example() {
    assert_eq!(count_black_tiles(input::EXAMPLE), 10);
}
