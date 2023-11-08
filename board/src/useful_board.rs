use serde::{Deserialize, Serialize};

use crate::Coordinate;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Game {
    pub board: Board,
    pub you_id: String,
    pub turn: u32,
}
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Board {
    pub width: u32,
    pub height: u32,
    pub snakes: Vec<Snake>,
    pub food: Vec<Coordinate>,
}
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Snake {
    pub id: String,
    pub body: Vec<Coordinate>,
    pub health: u8,
}
impl Snake {
    pub fn square_moves(coord: Coordinate) -> Vec<Coordinate> {
        vec![
            Coordinate::new(0, 1) + coord,
            Coordinate::new(0, -1) + coord,
            Coordinate::new(1, 0) + coord,
            Coordinate::new(-1, 0) + coord,
        ]
    }
}
