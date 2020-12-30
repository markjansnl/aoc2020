use aoc23::input;

fn labels_after_move(input: &str, moves: usize) -> String {
    let mut labels1 = input.bytes().map(|byte| byte - 48).collect();
    let mut labels2 = Vec::with_capacity(9);

    for _ in 0..moves / 2 {
        do_move(&mut labels1, &mut labels2);
        do_move(&mut labels2, &mut labels1);
    }

    String::from_utf8(
        labels1
            .iter()
            .skip_while(|i| **i != 1)
            .skip(1)
            .map(|i| *i + 48)
            .chain(labels1.iter().take_while(|i| **i != 1).map(|i| i + 48))
            .collect::<Vec<u8>>(),
    )
    .unwrap()
}

#[inline]
fn do_move(labels1: &mut Vec<u8>, labels2: &mut Vec<u8>) {
    let mut iter = labels1.iter();
    let current = *iter.next().unwrap();
    let three_cups: [u8; 3] = [
        *iter.next().unwrap(),
        *iter.next().unwrap(),
        *iter.next().unwrap(),
    ];
    let mut destination = if current == 1 { 9 } else { current - 1 };
    while three_cups.contains(&destination) {
        destination = if destination == 1 { 9 } else { destination - 1 };
    }
    labels2.clear();
    while let Some(cup) = iter.next() {
        labels2.push(*cup);
        if *cup == destination {
            labels2.push(three_cups[0]);
            labels2.push(three_cups[1]);
            labels2.push(three_cups[2]);
        }
    }
    labels2.push(current);
}

fn main() {
    println!("{}", labels_after_move(input::USER, 100));
}

#[test]
fn test_part1_example() {
    assert_eq!(labels_after_move(input::EXAMPLE, 10), "92658374");
    assert_eq!(labels_after_move(input::EXAMPLE, 100), "67384529");
}
