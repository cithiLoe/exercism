#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

#[derive(Debug)]
pub struct ChessPosition {
    x: i8,
    y: i8,
}

impl ChessPosition {
    pub fn new(x: i8, y: i8) -> Result<Self, &'static str> {
        if x < 0 || x > 7 || y < 0 || y > 7 {
            return Err("Queen position is off board");
        }
        Ok(ChessPosition { x: x, y: y })
    }

    fn same_row(&self, other: &ChessPosition) -> bool {
        self.x == other.x
    }

    fn same_column(&self, other: &ChessPosition) -> bool {
        self.y == other.y
    }

    fn same_diagonal(&self, other: &ChessPosition) -> bool {
        self.sum() == other.sum() || self.diff() == other.diff()
    }

    fn sum(&self) -> i8 {
        self.x + self.y
    }

    fn diff(&self) -> i8 {
        self.x - self.y
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position: position }
    }

    pub fn can_attack(self, other: &Queen) -> bool {
        if self.position.same_row(&other.position) ||
           self.position.same_column(&other.position) ||
           self.position.same_diagonal(&other.position) {
            true
        } else {
            false
        }
    }
}
