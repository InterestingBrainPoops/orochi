use serde::{Deserialize, Serialize};

use crate::movegen::{Move, MoveType};

#[derive(Clone, Serialize, Deserialize)]
pub struct Game {
    pub board: Board,
    pub you_id: usize,
    pub turn: u32,
    pub side: Side,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Board {
    pub width: u32,
    pub height: u32,
    pub snakes: Vec<Snake>,
    pub food: u128,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Snake {
    pub id: usize,
    pub alive: bool,
    pub body: Vec<u128>,
    pub full: u128,
    pub health: u8,
}

impl Snake {
    fn move_snake(&mut self, end_square: u128) {
        let rm = self.body.pop();
        if *self.body.last().unwrap() != rm.unwrap() {
            self.full ^= rm.unwrap();
        }
        self.body.insert(0, end_square);
        self.full |= end_square;
    }

    fn feed(&mut self) {
        self.body.push(*self.body.last().unwrap());
        self.health = 100;
    }
}
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Side {
    You,
    Them,
}

impl Game {
    pub fn step(&mut self, move_to_apply: &Move) {
        let end_square = if let MoveType::MoveSquare(square) = move_to_apply.move_type {
            square
        } else {
            self.board.snakes[move_to_apply.id].alive = false;
            return;
        };
        // player 1 is You, player 2 is Them.
        if self.side == Side::You {
            self.board.snakes[move_to_apply.id].move_snake(end_square);
            return;
        }
        // we do the actual fun calculations like deaths, food, etc.
        let other_id = move_to_apply.id;
        self.board.snakes[other_id].move_snake(end_square);
        for snake in &mut self.board.snakes {
            if snake.alive {
                snake.health -= 1;
            }
        }
        // feed snakes
        let mut eaten_food = 0;
        for snake in &mut self.board.snakes {
            if snake.alive && self.board.food & snake.body[0] == 1 {
                snake.feed();
                eaten_food |= snake.body[0];
            }
        }

        self.board.food ^= eaten_food;

        // out of bounds elims
        // can't happen thanks to movegen
        // health eliminations
        for snake in &mut self.board.snakes {
            if snake.health == 0 {
                snake.alive = false;
            }
        }

        // snake collision elims
        // specifically head to heads
        let mut dead = vec![];
        for snake in &self.board.snakes {
            if !snake.alive {
                continue;
            }
            for other in &self.board.snakes {
                if other.body[0] & snake.body[0] == 1 && other.body.len() >= snake.body.len() {
                    dead.push(snake.id);
                }
            }
        }
        for id in &dead {
            self.board.snakes[*id].alive = false;
        }
    }
    pub fn is_terminal(&self) -> bool {
        // What defines a terminal state in battlesnake?
        // Num of alive snakes <= 1
        self.board.snakes.iter().filter(|x| x.alive).count() <= 1 && Side::Them == self.side
    }
}
