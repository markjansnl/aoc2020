pub mod input;

#[derive(PartialEq)]
pub enum Square {
    Tree,
    Empty,
}

impl From<char> for Square {
    fn from(c: char) -> Square {
        use Square::*;
        match c {
            '#' => Tree,
            _ => Empty,
        }
    }
}

pub struct Map {
    squares: Vec<Vec<Square>>,
}

impl Map {
    pub fn width(&self) -> usize {
        self.squares[0].len()
    }

    pub fn height(&self) -> usize {
        self.squares.len()
    }

    pub fn count_trees(&self, right: usize, down: usize) -> usize {
        let mut x = 0usize;
        let mut y = 0usize;
        let mut trees = 0;

        while y < self.height() {
            if self.squares[y][x] == Square::Tree {
                trees += 1;
            }

            y += down;
            x = (x + right) % self.width();
        }

        trees
    }
}

impl From<&str> for Map {
    fn from(input: &str) -> Map {
        let squares = input
            .lines()
            .map(|line| line.chars().map(Square::from).collect())
            .collect();

        Map { squares }
    }
}
