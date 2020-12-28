use aoc20::{input, ConstructedImage, Image};

fn count_without_sea_monsters(input: &str) -> usize {
    let image: Image = input.into();
    let constructed_image: ConstructedImage = image.into();
    constructed_image.count_without_sea_monsters()
}

fn main() {
    // 2443 is too high
    println!("{}", count_without_sea_monsters(input::USER));
}

#[test]
fn test_part2_example() {
    assert_eq!(count_without_sea_monsters(input::EXAMPLE), 273);
}
