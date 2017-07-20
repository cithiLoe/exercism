// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: isize,
    y: isize,
    direction: Direction,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn turn_left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
}



impl Robot {
    #[allow(unused_variables)]
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot {
            x: x,
            y: y,
            direction: d,
        }
    }

    pub fn turn_right(self) -> Self {
        Robot {
            x: self.x,
            y: self.y,
            direction: self.direction.turn_right(),
        }
    }

    pub fn turn_left(self) -> Self {
        Robot {
            x: self.x,
            y: self.y,
            direction: self.direction.turn_left(),
        }
    }

    pub fn advance(self) -> Self {
        match self.direction {
            Direction::North => {
                Robot {
                    x: self.x,
                    y: self.y + 1,
                    direction: self.direction,
                }
            }
            Direction::South => {
                Robot {
                    x: self.x,
                    y: self.y - 1,
                    direction: self.direction,
                }
            }
            Direction::East => {
                Robot {
                    x: self.x + 1,
                    y: self.y,
                    direction: self.direction,
                }
            }
            Direction::West => {
                Robot {
                    x: self.x - 1,
                    y: self.y,
                    direction: self.direction,
                }
            }
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions
            .chars()
            .fold(self, |robot, instruction| match instruction {
                'R' => robot.turn_right(),
                'L' => robot.turn_left(),
                'A' => robot.advance(),
                _ => robot,
            })
    }

    pub fn position(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
