#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Position { x: x, y: y }
    }

    fn advance(self, direction: &Direction) -> Self {
        match *direction {
            Direction::North => Self::new(self.x, self.y + 1),
            Direction::South => Self::new(self.x, self.y - 1),
            Direction::East => Self::new(self.x + 1, self.y),
            Direction::West => Self::new(self.x - 1, self.y),
        }
    }

    fn as_tuple(&self) -> (isize, isize) {
        (self.x, self.y)
    }
}

pub struct Robot {
    position: Position,
    direction: Direction,
}

impl Direction {
    fn next_clockwise(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn prev_clockwise(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
}

impl Robot {
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot {
            position: Position { x: x, y: y },
            direction: d,
        }
    }

    pub fn turn_right(self) -> Self {
        Robot {
            position: self.position,
            direction: self.direction.next_clockwise(),
        }
    }

    pub fn turn_left(self) -> Self {
        Robot {
            position: self.position,
            direction: self.direction.prev_clockwise(),
        }
    }

    pub fn advance(self) -> Self {
        Robot {
            position: self.position.advance(&self.direction),
            direction: self.direction,
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
        self.position.as_tuple()
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
