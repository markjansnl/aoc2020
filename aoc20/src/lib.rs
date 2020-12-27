use std::{collections::HashMap, fmt};

pub mod input;

pub struct Image {
    tiles: Vec<Tile>,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Tile {
    id: u16,
    pixels: Vec<Vec<bool>>,
}

#[derive(Clone)]
enum BorderSide {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Clone)]
struct Border {
    id: usize,
    side: BorderSide,
    flipped: bool,
}

impl From<&str> for Image {
    fn from(input: &str) -> Self {
        Image {
            tiles: input
                .split("\n\n")
                .map(|tile_input| tile_input.into())
                .collect(),
        }
    }
}

impl Image {
    pub fn corners(&self) -> Vec<Tile> {
        let mut border_map: HashMap<usize, Vec<Tile>> = HashMap::new();
        let mut count_map: HashMap<Tile, usize> = HashMap::new();

        for tile in self.tiles.iter() {
            for border in tile.borders().iter() {
                border_map
                    .entry(border.id)
                    .and_modify(|vec| vec.push(tile.clone()))
                    .or_insert(vec![tile.clone()]);
            }
        }

        for (_, tiles) in border_map {
            if tiles.len() == 2 {
                for tile in tiles {
                    count_map
                        .entry(tile)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }
        }

        count_map
            .iter()
            .filter(|(_, count)| **count == 4) // Corders have 2 borders with an adjecent piece. Counted twice = 4
            .map(|(tile, _)| tile.clone())
            .collect()
    }
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

    fn borders(&self) -> Vec<Border> {
        let mut top = 0;
        let mut bottom = 0;
        let mut left = 0;
        let mut right = 0;

        // horizontal flipped
        let mut top_flipped = 0;
        let mut bottom_flipped = 0;
        let mut left_flipped = 0;
        let mut right_flipped = 0;

        self.pixels.iter().enumerate().for_each(|(index, line)| {
            if index == 0 {
                top = line
                    .iter()
                    .fold(0, |acc, pixel| (acc << 1) + if *pixel { 1 } else { 0 });
                top_flipped = line
                    .iter()
                    .rev()
                    .fold(0, |acc, pixel| (acc << 1) + if *pixel { 1 } else { 0 });
            }
            if index == 9 {
                bottom = line
                    .iter()
                    .rev()
                    .fold(0, |acc, pixel| (acc << 1) + if *pixel { 1 } else { 0 });
                bottom_flipped = line
                    .iter()
                    .fold(0, |acc, pixel| (acc << 1) + if *pixel { 1 } else { 0 });
            }
            left = left + (if *line.first().unwrap() { 1 } else { 0 } << index);
            left_flipped = left_flipped + (if *line.last().unwrap() { 1 } else { 0 } << index);

            right = (right << 1) + if *line.last().unwrap() { 1 } else { 0 };
            right_flipped = (right_flipped << 1) + if *line.first().unwrap() { 1 } else { 0 };
        });

        vec![
            Border {
                id: top,
                side: BorderSide::Top,
                flipped: false,
            },
            Border {
                id: bottom,
                side: BorderSide::Bottom,
                flipped: false,
            },
            Border {
                id: left,
                side: BorderSide::Left,
                flipped: false,
            },
            Border {
                id: right,
                side: BorderSide::Right,
                flipped: false,
            },
            Border {
                id: top_flipped,
                side: BorderSide::Top,
                flipped: true,
            },
            Border {
                id: bottom_flipped,
                side: BorderSide::Bottom,
                flipped: true,
            },
            Border {
                id: left_flipped,
                side: BorderSide::Left,
                flipped: true,
            },
            Border {
                id: right_flipped,
                side: BorderSide::Right,
                flipped: true,
            },
        ]
    }
}
