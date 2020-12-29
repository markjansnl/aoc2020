use std::collections::VecDeque;

use aoc22::input;

fn winning_score(input: &str) -> usize {
    let mut decks: Vec<VecDeque<u16>> = input
        .split("\n\n")
        .map(|deck| {
            deck.lines()
                .skip(1)
                .map(|card| card.parse().unwrap())
                .collect()
        })
        .collect();

    while !decks[0].is_empty() && !decks[1].is_empty() {
        let a = decks[0].pop_front().unwrap();
        let b = decks[1].pop_front().unwrap();
        if a > b {
            decks[0].push_back(a);
            decks[0].push_back(b);
        } else {
            decks[1].push_back(b);
            decks[1].push_back(a);
        }
    }
    
    decks
        .iter()
        .flatten()
        .rev()
        .enumerate()
        .map(|(i, card)| (i + 1) * *card as usize)
        .sum()
}

fn main() {
    println!("{}", winning_score(input::USER));
}

#[test]
fn test_part1_example() {
    assert_eq!(winning_score(input::EXAMPLE), 306);
}
