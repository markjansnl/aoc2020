use aoc20::{input, Image};

fn corner_tiles_product(input: &str) -> usize {
    let image: Image = input.into();
    image
        .corners()
        .iter()
        .map(|tile| tile.id() as usize)
        .product()
}

fn main() {
    println!("{}", corner_tiles_product(input::USER));
}

#[test]
fn test_part1_example() {
    assert_eq!(corner_tiles_product(input::EXAMPLE), 20899048083289);
}
