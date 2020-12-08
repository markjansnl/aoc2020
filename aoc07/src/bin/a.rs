use aoc07::input;
use std::collections::{HashMap, HashSet};

fn main() {
    println!("{}", shiny_gold_bags(input::USER));
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

                map.entry(contains_bag)
                    .and_modify(|vec| vec.push((quantity, bag.clone())))
                    .or_insert(vec![(quantity, bag.clone())]);
            } else {
                split.next();
                split.next();
            }
        }

        map
    })
}

fn shiny_gold_bags(input: &str) -> usize {
    let map = parse_input(input);
    contain(&map, &"shiny gold".to_string()).len()
}

fn contain(map: &HashMap<String, Vec<(usize, String)>>, start: &String) -> HashSet<String> {
    let mut set = HashSet::new();

    if let Some(vec) = map.get(start) {
        for (_, bag) in vec {
            set.insert(bag.clone());
            set.extend(contain(map, bag));
        }
    }
    set
}

#[test]
fn test_parse_input() {
    let map = parse_input(input::EXAMPLE);
    let vec = &map[&"shiny gold".to_string()];
    assert_eq!(vec.len(), 2);
}

#[test]
fn test_example() {
    assert_eq!(shiny_gold_bags(input::EXAMPLE), 4);
}
