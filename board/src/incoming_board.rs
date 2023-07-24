use serde::Deserialize;

use crate::{
    movegen::Move,
    useful_board::{Board, Game, Snake},
    zobrist::ZOBRIST_TABLE,
    Coordinate,
};

#[derive(Deserialize, Debug, Clone)]
pub struct Request {
    board: IBoard,
    you: IBattlesnake,
    turn: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct IBoard {
    width: u32,
    height: u32,
    food: Vec<Coordinate>,
    snakes: Vec<IBattlesnake>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct IBattlesnake {
    id: String,
    health: u8,
    body: Vec<Coordinate>,
}

impl Request {
    pub fn into_usable(&self) -> Game {
        let you_id = self
            .board
            .snakes
            .iter()
            .enumerate()
            .find(|x| x.1.id == self.you.id)
            .unwrap()
            .0;
        let mut hash = 0;
        let mut snakes = vec![];
        for (idx, snake) in self.board.snakes.iter().enumerate() {
            let mut body = vec![];
            let mut full = 0;
            for segment in &snake.body {
                let segment = segment.into_mask(self.board.width);
                hash ^= ZOBRIST_TABLE[segment.trailing_zeros() as usize];
                body.push(segment);
                full |= segment;
            }
            snakes.push(Snake {
                id: idx,
                alive: true,
                body,
                full,
                health: snake.health,
            })
        }
        let mut food = 0_u128;
        for food_square in &self.board.food {
            let food_square = food_square.into_mask(self.board.width);
            food |= food_square;
        }

        Game {
            you_id,
            turn: self.turn,
            board: Board {
                width: self.board.width,
                height: self.board.height,
                snakes,
                food,
            },
            hash,
        }
    }
}
