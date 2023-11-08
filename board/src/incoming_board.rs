use serde::Deserialize;

use crate::{
    useful_board::{Board, Game, Snake},
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
        let mut snakes = vec![];
        for snake in &self.board.snakes {
            snakes.push(Snake {
                id: snake.id.clone(),
                body: snake.body.clone(),
                health: snake.health,
            })
        }

        Game {
            you_id: self.you.id.clone(),
            turn: self.turn,
            board: Board {
                width: self.board.width,
                height: self.board.height,
                snakes,
                food: self.board.food.clone(),
            },
        }
    }
}
