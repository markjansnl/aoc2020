use aoc23::input;

fn labels_after_move(input: &str, moves: usize) -> usize {
    let mut labels: Vec<u32> = input
        .bytes()
        .map(|byte| byte as u32 - 48)
        .chain(10..=1_000_000)
        .collect();

    for i in 0..moves {
        if i % 10_000 == 0 {
            println!("{}", i);
        }
        let mut iter = labels.iter();
        let current = *iter.next().unwrap();
        let three_cups: [u32; 3] = [
            *iter.next().unwrap(),
            *iter.next().unwrap(),
            *iter.next().unwrap(),
        ];
        let mut destination = if current == 1 { 1_000_000 } else { current - 1 };
        while three_cups.contains(&destination) {
            destination = if destination == 1 {
                1_000_000
            } else {
                destination - 1
            };
        }
        labels.rotate_left(1);
        let (destination_index, _) = labels
            .iter()
            .enumerate()
            .find(|(_, cup)| **cup == destination)
            .unwrap();
        labels[0..=destination_index].rotate_left(3);
    }

    labels
        .into_iter()
        .skip_while(|i| *i != 1)
        .skip(1)
        .take(2)
        .map(|i| i as usize)
        .product()
}

fn main() {
    println!("{}", labels_after_move(input::USER, 10_000_000));
}

#[test]
fn test_part1_example() {
    assert_eq!(labels_after_move(input::EXAMPLE, 10_000_000), 149245887792);
}
