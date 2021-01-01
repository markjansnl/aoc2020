use std::collections::HashMap;

use aoc24::input;

fn black_tiles_after_days(input: &str, days: usize) -> usize {
    let mut tiles = tiles_from_input(input);
    for _ in 0..days {
        tiles = tiles_next_day(tiles);
    }
    tiles
        .iter()
        .filter(|(_, tile_is_black)| **tile_is_black)
        .count()
}

fn tiles_from_input(input: &str) -> HashMap<(i32, i32), bool> {
    input.lines().fold(
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
}

fn tiles_next_day(tiles: HashMap<(i32, i32), bool>) -> HashMap<(i32, i32), bool> {
    let mut map = HashMap::new();
    for ((y, x), tile_is_black) in tiles.iter() {
        if *tile_is_black {
            map.entry((*y, *x))
                .or_insert_with(|| next_day_tile(*y, *x, &tiles));
            for (neighbour_y, neighbour_x) in neighbours(*y, *x) {
                map.entry((neighbour_y, neighbour_x))
                    .or_insert_with(|| next_day_tile(neighbour_y, neighbour_x, &tiles));
            }
        }
    }
    map
}

#[inline]
fn next_day_tile(y: i32, x: i32, tiles: &HashMap<(i32, i32), bool>) -> bool {
    let tile_is_black = *tiles.get(&(y, x)).unwrap_or(&false);
    let black_neighbours = neighbours(y, x)
        .iter()
        .filter_map(|neighbour| tiles.get(neighbour))
        .filter(|neighbour_is_black| **neighbour_is_black)
        .count();
    if tile_is_black {
        black_neighbours == 1 || black_neighbours == 2
    } else {
        black_neighbours == 2
    }
}

fn neighbours(y: i32, x: i32) -> Vec<(i32, i32)> {
    let mut neighbours = Vec::new();
    for dy in -1..=1 {
        neighbours.push((y + dy, x + 1 - (dy * (y + 1)).abs() % 2));
        neighbours.push((y + dy, x - 1 + (dy * y).abs() % 2));
    }
    neighbours
}

fn main() {
    println!("{}", black_tiles_after_days(input::USER, 100));
}

#[test]
fn test_part2_example() {
    assert_eq!(black_tiles_after_days(input::EXAMPLE, 100), 2208);
}
