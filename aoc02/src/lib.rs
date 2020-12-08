pub mod input;

pub struct Input {
    min: usize,
    max: usize,
    char: char,
    password: String,
}

impl From<&str> for Input {
    fn from(input: &str) -> Input {
        let parts: Vec<&str> = input.split(&[' ', ':', '-'][..]).collect();
        Input {
            min: parts[0].parse().expect("Could not parse minimum"),
            max: parts[1].parse().expect("Could not parse maximem"),
            char: parts[2].chars().next().expect("Could not find character"),
            password: parts[4].to_string(),
        }
    }
}

impl Input {
    pub fn is_valid_a(&self) -> bool {
        let char_count = self.password.chars().filter(|c| *c == self.char).count();
        char_count >= self.min && char_count <= self.max
    }

    pub fn is_valid_b(&self) -> bool {
        let first = self.password.chars().nth(self.min - 1).expect("First character not found");
        let second = self.password.chars().nth(self.max - 1).expect("Second character not found");
        (first == self.char) ^ (second == self.char)
    }
}

pub fn count_valid_a(input: &str) -> usize {
    input.lines().filter(|line| Input::from(*line).is_valid_a()).count()
}

pub fn count_valid_b(input: &str) -> usize {
    input.lines().filter(|line| Input::from(*line).is_valid_b()).count()
}