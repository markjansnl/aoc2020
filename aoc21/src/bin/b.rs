use std::collections::{HashMap, HashSet};

use aoc21::input;

struct Food<'a> {
    ingredients: HashSet<&'a str>,
    allergenes: Vec<&'a str>,
}

impl<'a> From<&'a str> for Food<'a> {
    fn from(line: &'a str) -> Self {
        let food: Vec<&str> = line.trim_end_matches(')').split(" (contains ").collect();
        Food {
            ingredients: food[0].split(" ").collect(),
            allergenes: food[1].split(", ").collect(),
        }
    }
}

fn dangerous_ingredients(input: &str) -> String {
    let food_list: Vec<Food> = input.lines().map(|line| line.into()).collect();
    let all_ingredients: HashSet<&str> = food_list
        .iter()
        .map(|food| &food.ingredients)
        .flatten()
        .map(|s| *s)
        .collect();
    let all_allergenes: HashSet<&str> = food_list
        .iter()
        .map(|food| &food.allergenes)
        .flatten()
        .map(|s| *s)
        .collect();

    let mut allergene_ingredients: HashMap<&str, HashSet<&str>> = all_allergenes
        .iter()
        .map(|allergene| {
            (
                *allergene,
                food_list
                    .iter()
                    .filter(|food| food.allergenes.contains(allergene))
                    .map(|food| &food.ingredients)
                    .fold(
                        all_ingredients.clone(),
                        |allergene_ingredients, ingredients| {
                            allergene_ingredients
                                .intersection(ingredients)
                                .map(|s| *s)
                                .collect::<HashSet<&str>>()
                        },
                    ),
            )
        })
        .collect();

    let mut dangerous_ingredients: Vec<(&str, &str)> = Vec::new();
    while !allergene_ingredients.is_empty() {
        let allergene = *allergene_ingredients
            .iter()
            .find(|(_, ingredients)| ingredients.len() == 1)
            .unwrap()
            .0;

        let ingredient = allergene_ingredients
            .remove(allergene)
            .unwrap()
            .into_iter()
            .nth(0)
            .unwrap();

        for (_, ingredients) in allergene_ingredients.iter_mut() {
            ingredients.remove(ingredient);
        }

        dangerous_ingredients.push((allergene, ingredient));
    }

    dangerous_ingredients.sort_by(|(a, _), (b, _)| a.cmp(&b));
    dangerous_ingredients
        .iter()
        .map(|(_, ingredient)| *ingredient)
        .collect::<Vec<&str>>()
        .join(",")
}

fn main() {
    println!("{}", dangerous_ingredients(input::USER));
}

#[test]
fn test_part2_example() {
    assert_eq!(dangerous_ingredients(input::EXAMPLE), "mxmxvkd,sqjhc,fvjkl");
}
