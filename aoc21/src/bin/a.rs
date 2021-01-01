use std::collections::HashSet;

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

fn ingredients_without_allergenes(input: &str) -> usize {
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

    let allergene_ingredients: HashSet<&str> = all_allergenes
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
        .fold(HashSet::new(), |allergene_ingredients, (_, ingredients)| {
            allergene_ingredients
                .union(&ingredients)
                .map(|s| *s)
                .collect()
        });

    food_list
        .into_iter()
        .map(|food| {
            food.ingredients
                .into_iter()
                .filter(|ingredient| !allergene_ingredients.contains(*ingredient))
        })
        .flatten()
        .count()
}

fn main() {
    println!("{}", ingredients_without_allergenes(input::USER));
}

#[test]
fn test_part1_example() {
    assert_eq!(ingredients_without_allergenes(input::EXAMPLE), 5);
}
