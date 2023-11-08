use std::ops::Add;

use serde::{Deserialize, Serialize};

pub mod incoming_board;
pub mod useful_board;
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: i8,
    pub y: i8,
}

impl Coordinate {
    pub fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
