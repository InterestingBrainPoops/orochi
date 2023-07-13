use crate::movegen::Move;

#[derive(Clone)]
pub struct Game {
    pub board: Board,
    pub you_id: usize,
    pub turn: u32,
    pub side: Side,
}
#[derive(Clone)]
pub struct Board {
    pub width: u32,
    pub height: u32,
    pub snakes: Vec<Snake>,
    pub food: u128,
}
#[derive(Clone)]
pub struct Snake {
    pub id: usize,
    pub alive: bool,
    pub body: Vec<u128>,
    pub full: u128,
    pub health: u8,
}
#[derive(Clone, PartialEq, Eq)]
pub enum Side {
    You,
    Them,
}

impl Game {
    pub fn step(&mut self, move_to_apply: &Move) {
        todo!()
    }
    pub fn is_terminal(&self) -> bool {
        todo!()
    }
}
