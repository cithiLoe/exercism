extern crate num;

use num::Num;

pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T: Num + PartialOrd + PartialEq + Copy> Triangle<T> {
    fn valid(sides: [T; 3]) -> bool {
        sides.iter().all(|&s| s > T::zero()) && 
           (sides[0] + sides[1] >= sides[2]) && 
           (sides[1] + sides[2] >= sides[0]) && 
           (sides[2] + sides[0] >= sides[1])
    }

    pub fn build(sides: [T; 3]) -> Result<Self, ()> {
        if Triangle::valid(sides) {
            Ok(Triangle { sides: sides })
        } else {
            Err(())
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && 
        self.sides[0] == self.sides[2]
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[0] != self.sides[2] ||
        self.sides[0] == self.sides[2] && self.sides[0] != self.sides[1] ||
        self.sides[1] == self.sides[2] && self.sides[0] != self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        self.sides[0] != self.sides[1] && 
        self.sides[1] != self.sides[2] &&
        self.sides[0] != self.sides[2]
    }
}
