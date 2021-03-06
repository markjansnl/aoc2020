use std::{collections::HashMap, fmt, ops::Sub, vec};

pub mod input;

pub struct Image {
    tiles: Vec<Tile>,
}

#[derive(Clone)]
pub struct ConstructedImage {
    pixels: Vec<Vec<bool>>,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Tile {
    id: u16,
    pixels: Vec<Vec<bool>>,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum BorderSide {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Border {
    id: usize,
    side: BorderSide,
    flipped: bool,
}

impl BorderSide {
    fn angle(&self) -> u16 {
        match *self {
            BorderSide::Right => 0,
            BorderSide::Bottom => 90,
            BorderSide::Left => 180,
            BorderSide::Top => 270,
        }
    }
}

impl Sub<BorderSide> for BorderSide {
    type Output = u16;

    fn sub(self, rhs: BorderSide) -> Self::Output {
        (360 + self.angle() - rhs.angle()) % 360
    }
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
    pub fn size(&self) -> usize {
        (self.tiles.len() as f64).sqrt() as usize
    }

    pub fn corners(&self) -> Vec<Tile> {
        self.corners_from_border_map(&self.border_map())
    }

    fn corners_from_border_map(&self, border_map: &HashMap<usize, Vec<Tile>>) -> Vec<Tile> {
        let mut count_map: HashMap<Tile, usize> = HashMap::new();

        for (_, tiles) in border_map {
            for tile in tiles {
                count_map
                    .entry(tile.clone())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }

        count_map
            .iter()
            // Corners have 2 borders with an adjecent piece. Borders are counted from both sides, so are counted double (**count == 4)
            .filter(|(_, count)| **count == 4)
            .map(|(tile, _)| tile.clone())
            .collect()
    }

    fn border_map(&self) -> HashMap<usize, Vec<Tile>> {
        let mut border_map: HashMap<usize, Vec<Tile>> = HashMap::new();

        for tile in self.tiles.iter() {
            for border in tile.borders().iter() {
                border_map
                    .entry(border.id)
                    .and_modify(|vec| vec.push(tile.clone()))
                    .or_insert(vec![tile.clone()]);
            }
        }

        border_map.retain(|_, tiles| tiles.len() == 2);
        border_map
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
        let mut right = 0;
        let mut bottom = 0;
        let mut left = 0;
        let mut top = 0;

        let mut right_flipped = 0;
        let mut bottom_flipped = 0;
        let mut left_flipped = 0;
        let mut top_flipped = 0;

        self.pixels.iter().enumerate().for_each(|(index, line)| {
            if index == 9 {
                bottom = line
                    .iter()
                    .rev()
                    .fold(0, |acc, pixel| (acc << 1) + if *pixel { 1 } else { 0 });
                bottom_flipped = line
                    .iter()
                    .fold(0, |acc, pixel| (acc << 1) + if *pixel { 1 } else { 0 });
            }

            if index == 0 {
                top = line
                    .iter()
                    .fold(0, |acc, pixel| (acc << 1) + if *pixel { 1 } else { 0 });
                top_flipped = line
                    .iter()
                    .rev()
                    .fold(0, |acc, pixel| (acc << 1) + if *pixel { 1 } else { 0 });
            }

            right = (right << 1) + if *line.last().unwrap() { 1 } else { 0 };
            right_flipped = right_flipped + (if *line.last().unwrap() { 1 } else { 0 } << index);

            left = left + (if *line.first().unwrap() { 1 } else { 0 } << index);
            left_flipped = (left_flipped << 1) + if *line.first().unwrap() { 1 } else { 0 };
        });

        vec![
            Border {
                id: right,
                side: BorderSide::Right,
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
                id: top,
                side: BorderSide::Top,
                flipped: false,
            },
            Border {
                id: right_flipped,
                side: BorderSide::Right,
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
                id: top_flipped,
                side: BorderSide::Top,
                flipped: true,
            },
        ]
    }

    fn flip_rotate(&self, border_id: usize, side: BorderSide) -> Tile {
        let mut tile = self.clone();
        let border = self
            .borders()
            .into_iter()
            .filter(|border| border.id == border_id)
            .nth(0)
            .unwrap();
        tile = tile.rotate(side - border.side);
        if border.flipped {
            tile = tile.flip_horizontal();
            if side == BorderSide::Left || side == BorderSide::Right {
                tile = tile.rotate(180);
            }
        }
        tile
    }

    fn flip_horizontal(&self) -> Tile {
        let mut tile = self.clone();
        tile.pixels = self
            .pixels
            .iter()
            .cloned()
            .map(|line| line.iter().rev().cloned().collect())
            .collect();
        tile
    }

    fn rotate(&self, angle: u16) -> Tile {
        let mut tile = self.clone();
        for y in 0..10 {
            for x in 0..10 {
                tile.pixels[y][x] = match angle {
                    90 => self.pixels[9 - x][y].clone(),
                    180 => self.pixels[9 - y][9 - x].clone(),
                    270 => self.pixels[x][9 - y].clone(),
                    _ => self.pixels[y][x].clone(),
                }
            }
        }
        tile
    }

    fn pixels_without_border(&self) -> Vec<Vec<bool>> {
        self.pixels
            .iter()
            .skip(1)
            .take(8)
            .cloned()
            .map(|line| line.iter().skip(1).take(8).cloned().collect())
            .collect()
    }
}

impl From<Image> for ConstructedImage {
    fn from(image: Image) -> Self {
        let border_map = image.border_map();
        let first_corner = image
            .corners_from_border_map(&border_map)
            .into_iter()
            .find(|tile| {
                let borders = tile.borders();
                border_map.contains_key(&borders[0].id) && border_map.contains_key(&borders[1].id)
            })
            .unwrap();
        let mut tiles = vec![];
        let size = image.size();

        for y in 0..size {
            for x in 0..size {
                if x == 0 {
                    if y == 0 {
                        tiles.push(first_corner.clone());
                    } else {
                        let tile_above = &tiles[(y - 1) * size];
                        let tile_above_borders = tile_above.borders();
                        let tile_above_border_id =
                            if border_map.contains_key(&tile_above_borders[1].id) {
                                tile_above_borders[1].id
                            } else {
                                tile_above_borders[5].id
                            };
                        let tile = border_map[&tile_above_border_id]
                            .iter()
                            .find(|tile| tile.id != tile_above.id)
                            .unwrap();
                        tiles.push(
                            tile.flip_rotate(tile_above_borders[1].id, BorderSide::Top)
                                .flip_horizontal(),
                        );
                    }
                } else {
                    let tile_left = tiles.last().unwrap();
                    let tile_left_borders = tile_left.borders();
                    let tile_left_border_id = if border_map.contains_key(&tile_left_borders[0].id) {
                        tile_left_borders[0].id
                    } else {
                        tile_left_borders[4].id
                    };
                    let tile = border_map[&tile_left_border_id]
                        .iter()
                        .find(|tile| tile.id != tile_left.id)
                        .unwrap();
                    tiles.push(
                        tile.flip_rotate(tile_left_borders[0].id, BorderSide::Left)
                            .flip_horizontal()
                            .rotate(180),
                    );
                }
            }
        }

        let mut pixels = Vec::new();
        let mut tile_iter = tiles.iter().map(|tile| tile.pixels_without_border());
        for y in 0..size {
            for x in 0..size {
                if x == 0 {
                    for _ in 0..8 {
                        pixels.push(Vec::new());
                    }
                }
                let tile = tile_iter.next().unwrap();
                for (index, mut line) in tile.into_iter().enumerate() {
                    pixels[y * 8 + index].append(&mut line);
                }
            }
        }

        ConstructedImage { pixels }
    }
}

impl fmt::Debug for ConstructedImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConstructedImage")
            .field(
                "pixels",
                &self.as_string().split("\n").collect::<Vec<&str>>(),
            )
            .finish()
    }
}

impl ConstructedImage {
    pub fn count_without_sea_monsters(&self) -> usize {
        self.pixels
            .iter()
            .flatten()
            .filter(|pixel| **pixel)
            .count()
            - self.count_sea_monsters() * 15
    }

    fn count_sea_monsters(&self) -> usize {
        let mut image = self.clone();
        let size = image.pixels.len();
        let mut sea_monster_count = vec![];

        let mut sea_monster = vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, true, false,
        ];
        for _ in 0..size - 20 {
            sea_monster.push(false);
        }
        sea_monster.append(&mut vec![
            true, false, false, false, false, true, true, false, false, false, false, true, true,
            false, false, false, false, true, true, true,
        ]);
        for _ in 0..size - 20 {
            sea_monster.push(false);
        }
        sea_monster.append(&mut vec![
            false, true, false, false, true, false, false, true, false, false, true, false, false,
            true, false, false, true, false, false, false,
        ]);

        for i in 0..8 {
            let flatten_image = image
                .pixels
                .iter()
                .cloned()
                .flatten()
                .collect::<Vec<bool>>();
            let to = flatten_image.len() - sea_monster.len();
            sea_monster_count.push(
                (0..to)
                    .filter(|&i| {
                        flatten_image[i..i + sea_monster.len()]
                            .iter()
                            .zip(sea_monster.iter())
                            .all(|(a, b)| *a & *b == *b)
                    })
                    .count(),
            );

            image = image.rotate_right();
            if i == 3 {
                image = image.flip_horizontal();
            }
        }
        sea_monster_count.into_iter().max().unwrap()
    }

    fn as_string(&self) -> String {
        self.pixels.iter().fold(String::new(), |mut string, line| {
            for pixel in line {
                string.push(if *pixel { '#' } else { '.' });
            }
            string.push('\n');
            string
        })
    }

    fn flip_horizontal(&self) -> ConstructedImage {
        ConstructedImage {
            pixels: self
                .pixels
                .iter()
                .cloned()
                .map(|line| line.iter().rev().cloned().collect())
                .collect(),
        }
    }

    fn rotate_right(&self) -> ConstructedImage {
        let mut constructed_image = self.clone();
        let size = self.pixels.len();
        for y in 0..size {
            for x in 0..size {
                constructed_image.pixels[y][x] = self.pixels[size - 1 - x][y].clone();
            }
        }
        constructed_image
    }
}
