use aoc11::{input, Grid};

fn main() {
    println!("{}", run(input::USER));
}

fn run(input: &str) -> usize {
    let mut grid: Grid = input.into();
    loop {
        let next_gen = grid.next_generation_a();
        if next_gen == grid {
            return grid.occupied_seats();
        }
        grid = next_gen;
    }
}

#[test]
fn test_example() {
    assert_eq!(run(input::EXAMPLE), 37);
}
