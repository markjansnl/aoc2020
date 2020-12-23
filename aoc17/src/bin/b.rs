use aoc17::input;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Position {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl Position {
    pub fn new(x: isize, y: isize, z: isize, w: isize) -> Self {
        Position { x, y, z, w }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Grid {
    size: usize,
    layers: usize,
    vec: Vec<bool>,
}

impl Grid {
    pub fn next_cycle(&self) -> Self {
        let size = self.size + 2;
        let layers = self.layers + 1;
        let mut vec = Vec::new();

        for w in 0..layers {
            for z in 0..layers {
                for y in 0..size {
                    for x in 0..size {
                        let position =
                            Position::new(x as isize - 1, y as isize - 1, z as isize, w as isize);
                        let active_neighbours = self.active_neighbours(&position);
                        vec.push(match self.get(&position, 0, 0, 0, 0) {
                            true => active_neighbours == 2 || active_neighbours == 3,
                            false => active_neighbours == 3,
                        })
                    }
                }
            }
        }

        Grid { size, layers, vec }
    }

    pub fn cubes_active(&self) -> usize {
        let mut cubes_active = 0;
        for w in 0..self.layers {
            for z in 0..self.layers {
                for y in 0..self.size {
                    for x in 0..self.size {
                        cubes_active += if self.get(
                            &Position::new(x as isize, y as isize, z as isize, w as isize),
                            0,
                            0,
                            0,
                            0,
                        ) {
                            if z == 0 && w == 0 {
                                1
                            } else if z == 0 || w == 0 {
                                2
                            } else {
                                4
                            }
                        } else {
                            0
                        };
                    }
                }
            }
        }
        cubes_active
    }

    fn active_neighbours(&self, position: &Position) -> usize {
        let mut active_neighbours = 0usize;
        for delta_w in -1isize..=1 {
            for delta_z in -1isize..=1 {
                for delta_y in -1isize..=1 {
                    for delta_x in -1isize..=1 {
                        if !(delta_x == 0 && delta_y == 0 && delta_z == 0 && delta_w == 0) {
                            active_neighbours +=
                                if self.get(position, delta_x, delta_y, delta_z, delta_w) {
                                    1
                                } else {
                                    0
                                };
                            if active_neighbours == 4 {
                                return 4;
                            }
                        }
                    }
                }
            }
        }
        active_neighbours
    }

    #[inline]
    fn get(
        &self,
        position: &Position,
        delta_x: isize,
        delta_y: isize,
        delta_z: isize,
        delta_w: isize,
    ) -> bool {
        let x = position.x + delta_x;
        let y = position.y + delta_y;
        let z = (position.z + delta_z).abs() as usize;
        let w = (position.w + delta_w).abs() as usize;
        if x >= 0
            && (x as usize) < self.size
            && y >= 0
            && (y as usize) < self.size
            && z < self.layers
            && w < self.layers
        {
            let result = *self
                .vec
                .get(
                    w * self.layers * self.size * self.size
                        + z as usize * self.size * self.size
                        + y as usize * self.size
                        + x as usize,
                )
                .unwrap_or(&false);
            result
        } else {
            false
        }
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let mut vec = Vec::new();
        for line in input.lines() {
            for byte in line.bytes() {
                vec.push(byte == b'#');
            }
        }
        Grid {
            size: input.lines().nth(0).unwrap().bytes().count(),
            layers: 1,
            vec,
        }
    }
}

fn run(input: &str) -> usize {
    let mut grid: Grid = input.into();
    for _ in 0..6 {
        grid = grid.next_cycle();
    }
    grid.cubes_active()
}

fn main() {
    println!("{}", run(input::USER));
}

#[test]
fn test_part2_example() {
    assert_eq!(run(input::EXAMPLE), 848);
}
