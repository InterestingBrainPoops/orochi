use serde::Deserialize;

pub mod incoming_board;
pub mod movegen;
pub mod useful_board;

#[derive(Deserialize, Clone, Copy, Debug)]
pub struct Coordinate {
    pub x: i8,
    pub y: i8,
}

impl Coordinate {
    fn into_mask(&self, width: u32) -> u128 {
        1 << (self.x as u32 + width * self.y as u32)
    }
}

static UNIVERSE: u128 = u128::MAX >> (128 - 121);
