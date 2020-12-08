use aoc07::input;
use std::collections::HashMap;

fn main() {
    println!("{}", shiny_gold_bag_contents(input::USER));
}

pub fn parse_input(input: &str) -> HashMap<String, Vec<(usize, String)>> {
    input.lines().fold(HashMap::new(), |mut map, line| {
        let mut split = line.split(" ").peekable();
        let color1 = split.next().unwrap();
        let color2 = split.next().unwrap();
        let bag = format!("{} {}", color1, color2);
        split.next();
        split.next();

        while split.peek().is_some() {
            let quantity = split.next().unwrap().parse().unwrap_or(0usize);
            if quantity > 0 {
                let color3 = split.next().unwrap();
                let color4 = split.next().unwrap();
                let contains_bag = format!("{} {}", color3, color4);
                split.next();

                map.entry(bag.clone())
                    .and_modify(|vec| vec.push((quantity, contains_bag.clone())))
                    .or_insert(vec![(quantity, contains_bag)]);
            } else {
                split.next();
                split.next();
            }
        }

        map
    })
}

fn shiny_gold_bag_contents(input: &str) -> usize {
    let map = parse_input(input);
    contains(&map, &"shiny gold".to_string())
}

fn contains(map: &HashMap<String, Vec<(usize, String)>>, start: &String) -> usize {
    map.get(start)
        .unwrap_or(&vec![])
        .iter()
        .fold(0, |acc, (quantity, bag)| {
            acc + quantity + quantity * contains(map, bag)
        })
}

#[test]
fn test_parse_input() {
    let map = parse_input(input::EXAMPLE);
    let vec = &map[&"shiny gold".to_string()];

    assert_eq!(vec.len(), 2);
    assert_eq!(
        vec.iter()
            .fold(0usize, |acc, (quantity, _)| acc + *quantity),
        3
    );
}

#[test]
fn test_example() {
    assert_eq!(shiny_gold_bag_contents(input::EXAMPLE), 32);
    assert_eq!(shiny_gold_bag_contents(input::EXAMPLE2), 126);
}
