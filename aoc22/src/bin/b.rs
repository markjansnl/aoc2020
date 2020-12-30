use std::{
    collections::{HashSet, VecDeque},
    vec,
};

use aoc22::input;

fn winning_score(input: &str) -> usize {
    let mut decks: Vec<VecDeque<u8>> = input
        .split("\n\n")
        .map(|deck| {
            deck.lines()
                .skip(1)
                .map(|card| card.parse().unwrap())
                .collect()
        })
        .collect();

    play(&mut decks);

    decks
        .iter()
        .flatten()
        .rev()
        .enumerate()
        .map(|(i, card)| (i + 1) * *card as usize)
        .sum()
}

fn play(decks: &mut Vec<VecDeque<u8>>) {
    let mut rounds = HashSet::new();
    while !decks[0].is_empty() && !decks[1].is_empty() {
        if rounds.contains(decks) {
            decks[1].clear();
            return;
        }
        rounds.insert(decks.clone());

        let a = decks[0].pop_front().unwrap();
        let b = decks[1].pop_front().unwrap();

        if if a as usize <= decks[0].len() && b as usize <= decks[1].len() {
            let mut subgame_decks = vec![
                decks[0].iter().take(a as usize).cloned().collect(),
                decks[1].iter().take(b as usize).cloned().collect(),
            ];
            play(&mut subgame_decks);
            subgame_decks[1].is_empty()
        } else {
            a > b
        } {
            decks[0].push_back(a);
            decks[0].push_back(b);
        } else {
            decks[1].push_back(b);
            decks[1].push_back(a);
        }
    }
}

fn main() {
    println!("{}", winning_score(input::USER));
}

#[test]
fn test_part2_example() {
    assert_eq!(winning_score(input::EXAMPLE), 291);
}
