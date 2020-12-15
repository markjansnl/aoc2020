use core::panic;

pub mod input;

pub struct Instruction {
    action: u8,
    value: isize,
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        Instruction {
            action: line.as_bytes()[0],
            value: line[1..].parse().unwrap(),
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| line.into()).collect()
}

#[derive(Default)]
pub struct Ship {
    x: isize,
    y: isize,
    direction: isize,
}

impl Ship {
    pub fn apply(&mut self, instruction: &Instruction) {
        match instruction.action {
            b'N' => self.y += instruction.value,
            b'S' => self.y -= instruction.value,
            b'E' => self.x += instruction.value,
            b'W' => self.x -= instruction.value,
            b'L' => self.direction = (self.direction + 360 - instruction.value) % 360,
            b'R' => self.direction = (self.direction + instruction.value) % 360,
            b'F' => self.apply(&Instruction {
                action: match self.direction {
                    0 => b'E',
                    90 => b'S',
                    180 => b'W',
                    270 => b'N',
                    _ => panic!("Invalid direction"),
                },
                value: instruction.value,
            }),
            _ => panic!("Invalid input"),
        }
    }

    pub fn manhatten_distance(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

pub struct Waypoint {
    x: isize,
    y: isize,
    ship: Ship,
}

impl Waypoint {
    pub fn new() -> Self {
        Waypoint {
            x: 10,
            y: 1,
            ship: Ship::default(),
        }
    }

    pub fn apply(&mut self, instruction: &Instruction) {
        match instruction.action {
            b'N' => self.y += instruction.value,
            b'S' => self.y -= instruction.value,
            b'E' => self.x += instruction.value,
            b'W' => self.x -= instruction.value,
            b'L' => self.apply(&Instruction {
                action: b'R',
                value: 360 - instruction.value,
            }),
            b'R' => {
                let (x, y) = match instruction.value % 360 {
                    0 => (self.x, self.y),
                    90 => (self.y, -self.x),
                    180 => (-self.x, -self.y),
                    270 => (-self.y, self.x),
                    _ => panic!("Invalid  angle"),
                };
                self.x = x;
                self.y = y;
            }
            b'F' => {
                self.ship.apply(&Instruction {
                    action: b'E',
                    value: instruction.value * self.x,
                });
                self.ship.apply(&Instruction {
                    action: b'N',
                    value: instruction.value * self.y,
                });
            }
            _ => panic!("Invalid input"),
        }
    }

    pub fn manhatten_distance(&self) -> isize {
        self.ship.manhatten_distance()
    }

    pub fn print(&self) {
        println!("{}, {}, {}, {}", self.x, self.y, self.ship.x, self.ship.y);
    }
}
