use std::collections::HashMap;

pub mod input;

#[derive(Debug)]
pub enum Rule {
    Char(u8),
    SubRules(Vec<Vec<u8>>),
}

pub struct Rules {
    rules: HashMap<u8, Rule>,
}

impl From<&str> for Rules {
    fn from(rules_input: &str) -> Self {
        Rules {
            rules: rules_input
                .lines()
                .map(|line| {
                    let mut line_split = line.split(": ");
                    let rule_nr = line_split.next().unwrap().parse::<u8>().unwrap();
                    let rule = line_split.next().unwrap();
                    (
                        rule_nr,
                        if rule.as_bytes()[0] == b'"' {
                            Rule::Char(rule.as_bytes()[1])
                        } else {
                            Rule::SubRules(
                                rule.split("| ")
                                    .map(|subrules| {
                                        subrules
                                            .split_terminator(" ")
                                            .map(|subrule_nr| subrule_nr.parse::<u8>().unwrap())
                                            .collect::<Vec<u8>>()
                                    })
                                    .collect::<Vec<Vec<u8>>>(),
                            )
                        },
                    )
                })
                .collect::<HashMap<u8, Rule>>(),
        }
    }
}

impl Rules {
    pub fn insert(&mut self, rule_nr: u8, rule: Rule) -> Option<Rule> {
        self.rules.insert(rule_nr, rule)
    }

    pub fn validate_line(&self, line: &str, start_index: usize, rule_nr: u8) -> Option<usize> {
        if start_index >= line.len() {
            return None;
        }

        let rule = self.rules.get(&rule_nr).unwrap();
        let valid = match rule {
            Rule::Char(char) => {
                if line.as_bytes()[start_index] == *char {
                    Some(1)
                } else {
                    None
                }
            }
            Rule::SubRules(subrules) => subrules
                .iter()
                .filter_map(|subrule| {
                    subrule.iter().try_fold(0, |acc, subrule_nr| {
                        self.validate_line(&line, start_index + acc, *subrule_nr)
                            .map(|eaten| acc + eaten)
                    })
                })
                .next(),
                // Both left and right submatch are valid. In some cases we need the other one, so more
                // are valid than what now is calculated
        };
        valid
    }
}
