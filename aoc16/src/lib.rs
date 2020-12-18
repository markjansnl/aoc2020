use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

pub mod input;

pub struct Field {
    name: String,
    ranges: Vec<RangeInclusive<usize>>,
}

impl Field {
    pub fn contains(&self, number: usize) -> bool {
        self.ranges.iter().any(|range| range.contains(&number))
    }
}

pub struct Input {
    fields: Vec<Field>,
    my_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

impl From<&str> for Input {
    fn from(input: &str) -> Self {
        let input_parts: Vec<&str> = input.split("\n\n").collect();

        let fields = input_parts[0]
            .lines()
            .map(|line| {
                let field_split: Vec<&str> = line.split(": ").collect();

                Field {
                    name: field_split[0].to_string(),
                    ranges: field_split[1]
                        .split(" or ")
                        .map(|range| {
                            let range_split: Vec<usize> =
                                range.split("-").map(|str| str.parse().unwrap()).collect();
                            range_split[0]..=range_split[1]
                        })
                        .collect(),
                }
            })
            .collect();

        Input {
            fields,

            my_ticket: input_parts[1]
                .lines()
                .nth(1)
                .unwrap()
                .split(",")
                .map(|number| number.parse().unwrap())
                .collect(),

            nearby_tickets: input_parts[2]
                .lines()
                .skip(1)
                .map(|line| {
                    line.split(",")
                        .map(|number| number.parse().unwrap())
                        .collect()
                })
                .collect(),
        }
    }
}

impl Input {
    pub fn ticket_scanning_error_rate(&self) -> usize {
        self.nearby_tickets
            .iter()
            .flatten()
            .filter(|number| self.fields.iter().all(|field| !field.contains(**number)))
            .sum()
    }

    pub fn departure(&self) -> usize {
        let valid_nearby_tickets: Vec<&Vec<usize>> = self
            .nearby_tickets
            .iter()
            .filter(|nearby_ticket| {
                nearby_ticket
                    .iter()
                    .all(|number| self.fields.iter().any(|field| field.contains(*number)))
            })
            .collect();

        let mut valid_fields: HashMap<usize, HashSet<String>> = self
            .my_ticket
            .iter()
            .enumerate()
            .map(|(index, my_ticket_number)| {
                (
                    *my_ticket_number,
                    self.fields
                        .iter()
                        .filter(|field| {
                            field.contains(*my_ticket_number)
                                && valid_nearby_tickets
                                    .iter()
                                    .all(|nearby_ticket| field.contains(nearby_ticket[index]))
                        })
                        .map(|field| field.name.clone())
                        .collect(),
                )
            })
            .collect();

        let mut departure_product = 1;
        while !valid_fields.is_empty() {
            let mut _my_ticket_number = 0;
            {
                _my_ticket_number = *valid_fields
                    .iter()
                    .find(|(_, fields)| fields.len() == 1)
                    .unwrap()
                    .0;
            }

            let field_name = valid_fields
                .remove(&_my_ticket_number)
                .unwrap()
                .into_iter()
                .nth(0)
                .unwrap();

            for (_, fields) in valid_fields.iter_mut() {
                fields.remove(&field_name.clone());
            }

            if field_name.starts_with("departure") {
                departure_product *= _my_ticket_number;
            }
        }

        departure_product
    }
}
