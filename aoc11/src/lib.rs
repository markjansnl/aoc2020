use std::collections::HashMap;

pub mod input;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
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

#[derive(PartialEq, Eq)]
pub struct Grid {
    width: usize,
    height: usize,
    map: HashMap<Position, Seat>,
}

impl Grid {
    pub fn next_generation_a(&self) -> Self {
        let mut map = HashMap::new();
        for (position, seat) in self.map.iter() {
            map.insert(
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
            );
        }
        Grid {
            width: self.width,
            height: self.height,
            map,
        }
    }

    pub fn next_generation_b(&self) -> Self {
        let mut map = HashMap::new();
        for (position, seat) in self.map.iter() {
            map.insert(
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
            );
        }
        Grid {
            width: self.width,
            height: self.height,
            map,
        }
    }

    pub fn occupied_seats(&self) -> usize {
        self.map
            .iter()
            .filter(|(_, &seat)| seat == Seat::Occupied)
            .count()
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
        if x < 0 || y < 0 {
            Seat::Floor
        } else {
            *self
                .map
                .get(&Position::new(x as usize, y as usize))
                .unwrap_or(&Seat::Floor)
        }
    }

    fn get_first_seen(&self, position: &Position, delta_x: isize, delta_y: isize) -> Seat {
        let mut x = position.x as isize + delta_x;
        let mut y = position.y as isize + delta_y;
        while x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height {
            let seat = *self
                .map
                .get(&Position::new(x as usize, y as usize))
                .unwrap_or(&Seat::Floor);
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
        let mut map = HashMap::new();
        let mut height = 0;
        for (y, line) in input.lines().enumerate() {
            for (x, byte) in line.bytes().enumerate() {
                map.insert(Position::new(x, y), byte.into());
            }
            height += 1;
        }
        Grid {
            width: input.lines().nth(0).unwrap().bytes().count(),
            height,
            map,
        }
    }
}
