use aoc17::input;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Position {
    x: isize,
    y: isize,
    z: isize,
}

impl Position {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Position { x, y, z }
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

        for z in 0..layers {
            for y in 0..size {
                for x in 0..size {
                    let position = Position::new(x as isize - 1, y as isize - 1, z as isize);
                    let active_neighbours = self.active_neighbours(&position);
                    vec.push(match self.get(&position, 0, 0, 0) {
                        true => active_neighbours == 2 || active_neighbours == 3,
                        false => active_neighbours == 3,
                    })
                }
            }
        }

        Grid { size, layers, vec }
    }

    pub fn cubes_active(&self) -> usize {
        let all_layers = self.vec.iter().filter(|active| **active).count();

        let layer0 = self.vec[0..self.size * self.size]
            .iter()
            .filter(|active| **active)
            .count();

        all_layers * 2 - layer0
    }

    pub fn print(&self) {
        for (index, active) in self.vec.iter().enumerate() {
            if index % self.size == 0 {
                println!();
                if index % (self.size * self.size) == 0 {
                    println!("z={}", index / self.size / self.size);
                }
            }
            print!(
                "{}",
                match active {
                    true => "#",
                    false => ".",
                }
            );
        }
        println!();
    }

    fn active_neighbours(&self, position: &Position) -> usize {
        let mut active_neighbours = 0usize;
        for delta_z in -1isize..=1 {
            for delta_y in -1isize..=1 {
                for delta_x in -1isize..=1 {
                    if !(delta_x == 0 && delta_y == 0 && delta_z == 0) {
                        active_neighbours += if self.get(position, delta_x, delta_y, delta_z) {
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
        active_neighbours
    }

    #[inline]
    fn get(&self, position: &Position, delta_x: isize, delta_y: isize, delta_z: isize) -> bool {
        let x = position.x as isize + delta_x;
        let y = position.y as isize + delta_y;
        let z = (position.z as isize + delta_z).abs() as usize;
        if x >= 0
            && (x as usize) < self.size
            && y >= 0
            && (y as usize) < self.size
            && z < self.layers
        {
            let result = *self
                .vec
                .get(z * self.size * self.size + y as usize * self.size + x as usize)
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
    grid.print();
    for _ in 0..6 {
        grid = grid.next_cycle();
        grid.print();
    }
    grid.cubes_active()
}

fn main() {
    println!("{}", run(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(run(input::EXAMPLE), 112);
}
