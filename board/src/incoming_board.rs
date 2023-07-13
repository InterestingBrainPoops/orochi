use serde::Deserialize;

use crate::{
    useful_board::{Board, Game, Side, Snake},
    Coordinate,
};

#[derive(Deserialize)]
pub struct Request {
    board: IBoard,
    you: IBattlesnake,
    turn: u32,
}

#[derive(Deserialize)]
pub struct IBoard {
    width: u32,
    height: u32,
    food: Vec<Coordinate>,
    snakes: Vec<IBattlesnake>,
}
#[derive(Deserialize)]
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
        let mut snakes = vec![];
        for (idx, snake) in self.board.snakes.iter().enumerate() {
            let mut body = vec![];
            let mut full = 0;
            for segment in &snake.body {
                let segment = segment.into_mask(self.board.width);
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
            food |= food_square.into_mask(self.board.width);
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
            side: Side::You,
        }
    }
}
