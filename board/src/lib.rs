use std::ops::Add;

use serde::Deserialize;

pub mod incoming_board;
pub mod movegen;
pub mod useful_board;

#[derive(Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: i8,
    pub y: i8,
}

impl Coordinate {
    pub fn into_mask(&self, width: u32) -> u128 {
        1 << (self.x as u32 + width * self.y as u32)
    }
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

impl From<u128> for Coordinate {
    fn from(value: u128) -> Self {
        Self {
            y: value.trailing_zeros() as i8 / 11,
            x: value.trailing_zeros() as i8 % 11,
        }
    }
}
static UNIVERSE: u128 = u128::MAX >> (128 - 121);
