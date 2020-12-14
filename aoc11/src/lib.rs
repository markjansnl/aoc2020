pub mod input;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Seat {
    Empty,
    Occupied,
    Floor,
}

impl From<u8> for Seat {
    fn from(byte: u8) -> Self {
        match byte {
            b'L' => Seat::Empty,
            b'#' => Seat::Occupied,
            _ => Seat::Floor,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Grid {
    width: usize,
    height: usize,
    vec: Vec<(Position, Seat)>,
}

impl Grid {
    pub fn next_generation_a(&self) -> Self {
        let mut vec = Vec::new();
        for (position, seat) in self.vec.iter() {
            vec.push((
                *position,
                match seat {
                    Seat::Empty => {
                        if self.occupied_adjecent_seats_a(position) == 0 {
                            Seat::Occupied
                        } else {
                            Seat::Empty
                        }
                    }
                    Seat::Occupied => {
                        if self.occupied_adjecent_seats_a(position) < 4 {
                            Seat::Occupied
                        } else {
                            Seat::Empty
                        }
                    }
                    Seat::Floor => Seat::Floor,
                },
            ));
        }
        Grid {
            width: self.width,
            height: self.height,
            vec,
        }
    }

    pub fn next_generation_b(&self) -> Self {
        let mut vec = Vec::new();
        for (position, seat) in self.vec.iter() {
            vec.push((
                *position,
                match seat {
                    Seat::Empty => {
                        if self.occupied_adjecent_seats_b(position) == 0 {
                            Seat::Occupied
                        } else {
                            Seat::Empty
                        }
                    }
                    Seat::Occupied => {
                        if self.occupied_adjecent_seats_b(position) < 5 {
                            Seat::Occupied
                        } else {
                            Seat::Empty
                        }
                    }
                    Seat::Floor => Seat::Floor,
                },
            ));
        }
        Grid {
            width: self.width,
            height: self.height,
            vec,
        }
    }

    pub fn occupied_seats(&self) -> usize {
        self.vec
            .iter()
            .filter(|(_, seat)| *seat == Seat::Occupied)
            .count()
    }

    pub fn print(&self) {
        for (position, seat) in self.vec.iter() {
            if position.x == 0 {
                println!();
            }
            print!("{}", match seat {
                Seat::Empty => "L",
                Seat::Occupied => "#",
                Seat::Floor => ".",
            });
        }
        println!();
    }

    fn occupied_adjecent_seats_a(&self, position: &Position) -> usize {
        [
            self.get(position, -1, -1),
            self.get(position, -1, 0),
            self.get(position, -1, 1),
            self.get(position, 0, -1),
            self.get(position, 0, 1),
            self.get(position, 1, -1),
            self.get(position, 1, 0),
            self.get(position, 1, 1),
        ]
        .iter()
        .filter(|&&seat| seat == Seat::Occupied)
        .count()
    }

    fn occupied_adjecent_seats_b(&self, position: &Position) -> usize {
        [
            self.get_first_seen(position, -1, -1),
            self.get_first_seen(position, -1, 0),
            self.get_first_seen(position, -1, 1),
            self.get_first_seen(position, 0, -1),
            self.get_first_seen(position, 0, 1),
            self.get_first_seen(position, 1, -1),
            self.get_first_seen(position, 1, 0),
            self.get_first_seen(position, 1, 1),
        ]
        .iter()
        .filter(|&&seat| seat == Seat::Occupied)
        .count()
    }

    fn get(&self, position: &Position, delta_x: isize, delta_y: isize) -> Seat {
        let x = position.x as isize + delta_x;
        let y = position.y as isize + delta_y;
        if x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height {
            self
                .vec
                .get(y as usize * self.width + x as usize)
                .unwrap_or(&(Position::new(0, 0), Seat::Floor)).1
        } else {
            Seat::Floor
        }
    }

    fn get_first_seen(&self, position: &Position, delta_x: isize, delta_y: isize) -> Seat {
        let mut x = position.x as isize + delta_x;
        let mut y = position.y as isize + delta_y;
        while x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height {
            let seat = self
                .vec
                .get(y as usize * self.width + x as usize)
                .unwrap_or(&(Position::new(0, 0), Seat::Floor)).1;
        if seat != Seat::Floor {
                return seat;
            }
            x += delta_x;
            y += delta_y;
        }
        Seat::Floor
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let mut vec = Vec::new();
        let mut height = 0;
        for (y, line) in input.lines().enumerate() {
            for (x, byte) in line.bytes().enumerate() {
                vec.push((Position::new(x, y), byte.into()));
            }
            height += 1;
        }
        Grid {
            width: input.lines().nth(0).unwrap().bytes().count(),
            height,
            vec,
        }
    }
}
