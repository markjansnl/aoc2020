use std::collections::HashMap;

use aoc20::{input, parse_input};

fn corner_tiles_product(input: &str) -> usize {
    let tiles = parse_input(input);
    let mut border_map: HashMap<usize, Vec<u16>> = HashMap::new();
    let mut count_map: HashMap<u16, usize> = HashMap::new();

    for tile in tiles {
        let borders = tile.borders();
        let flipped_borders = borders.iter().map(|border| {
            let mut border = *border;
            let mut flipped_border = 0;
            for _ in 0..10 {
                flipped_border = (flipped_border << 1) + (border % 2);
                border = border >> 1;
            }
            flipped_border
        }).collect::<Vec<usize>>();

        for border in borders.iter().chain(flipped_borders.iter()) {
            border_map.entry(*border).and_modify(|vec| vec.push(tile.id())).or_insert(vec![tile.id()]);
        }
    }

    for (_, tiles) in border_map {
        if tiles.len() == 2 {
            for tile_id in tiles {
                count_map.entry(tile_id).and_modify(|count| *count += 1).or_insert(1);
            }
        }
    }

    count_map.iter().filter(|(_, count)| **count == 4).map(|(tile_id, _)| *tile_id as usize).product()
}

fn main() {
    println!("{}", corner_tiles_product(input::USER));
}

#[test]
fn test_part1_example() {
    assert_eq!(corner_tiles_product(input::EXAMPLE), 20899048083289);
}
