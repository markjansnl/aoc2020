use std::fmt;

pub mod input;

pub struct Tile {
    id: u16,
    pixels: Vec<Vec<bool>>,
}

impl From<&str> for Tile {
    fn from(tile_input: &str) -> Self {
        let mut lines = tile_input.lines();
        let id = lines.next().unwrap()[5..9].parse().unwrap();

        let mut pixels = Vec::new();
        while let Some(line) = lines.next() {
            pixels.push(line.bytes().map(|byte| byte == b'#').collect())
        }

        Tile { id, pixels }
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pixels = self
            .pixels
            .iter()
            .map(|line| {
                line.iter().fold(String::new(), |mut pixel_line, pixel| {
                    pixel_line.push(if *pixel { '#' } else { '.' });
                    pixel_line
                })
            })
            .collect::<Vec<String>>();

        f.debug_struct("Tile")
            .field("id", &self.id)
            .field("pixels", &pixels)
            .finish()
    }
}

impl Tile {
    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn borders(&self) -> Vec<usize> {
        let mut top = 0;
        let mut bottom = 0;
        let mut left = 0;
        let mut right = 0;

        self.pixels.iter().enumerate().for_each(|(index, line)| {
            if index == 0 {
                top = line
                    .iter()
                    .fold(0, |acc, pixel| (acc << 1) + if *pixel { 1 } else { 0 });
            }
            if index == 9 {
                bottom = line
                    .iter()
                    .rev()
                    .fold(0, |acc, pixel| (acc << 1) + if *pixel { 1 } else { 0 });
            }
            left = left + (if *line.first().unwrap() { 1 } else { 0 } << index);
            right = (right << 1) + if *line.last().unwrap() { 1 } else { 0 };
        });

        vec![top, bottom, left, right]
    }
}

pub fn parse_input(input: &str) -> Vec<Tile> {
    input
        .split("\n\n")
        .map(|tile_input| tile_input.into())
        .collect()
}
