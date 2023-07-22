use serde::{Deserialize, Serialize};

use crate::movegen::{Move, MoveType};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Game {
    pub board: Board,
    pub you_id: usize,
    pub turn: u32,
}
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Board {
    pub width: u32,
    pub height: u32,
    pub snakes: Vec<Snake>,
    pub food: u128,
}
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Snake {
    pub id: usize,
    pub alive: bool,
    pub body: Vec<u128>,
    pub full: u128,
    pub health: u8,
}

impl Snake {
    fn move_snake(&mut self, end_square: u128) -> u128 {
        let rm = self.body.pop().unwrap();
        if *self.body.last().unwrap() != rm {
            self.full ^= rm;
        }
        self.body.insert(0, end_square);
        self.full |= end_square;
        rm
    }
    fn unmove(&mut self, tail: u128) {
        self.full ^= self.body.remove(0);
        self.full |= tail;
        self.body.push(tail);
    }
    fn unfeed(&mut self, health: u8) {
        let rm = self.body.pop().unwrap();
        if *self.body.last().unwrap() != rm {
            self.full ^= rm;
        }
        self.health = health;
    }
    fn feed(&mut self) {
        self.body.push(*self.body.last().unwrap());
        self.health = 100;
    }
}

#[derive(Clone, Debug)]
pub struct Undo {
    eaten_food: u128,
    tails: Vec<(usize, u128)>,
    kills: Vec<usize>,
    snake_eats: Vec<(usize, u8)>,
}
impl Game {
    pub fn undo(&mut self, undo: &Undo) {
        // bring back the dead
        for id in &undo.kills {
            self.board.snakes[*id].alive = true;
        }
        // put the eaten food back
        self.board.food |= undo.eaten_food;
        // unfeed the snakes
        for (id, health) in &undo.snake_eats {
            self.board.snakes[*id].unfeed(*health);
        }
        for snake in &mut self.board.snakes {
            snake.health += 1;
        }
        // unmove the snakes
        for (id, tail) in &undo.tails {
            self.board.snakes[*id].unmove(*tail);
        }
    }
    pub fn step(&mut self, moves: &Vec<Move>) -> Undo {
        let mut out = Undo {
            eaten_food: 0,
            tails: vec![],
            kills: vec![],
            snake_eats: vec![],
        };
        // 1. determine if any snakes died, and kill them
        // 2. move the snakes
        // 3. remove health from the living
        // 4. feed the snakes
        // skip out of bound elims
        // 6. health elims
        // 7. skip body elims
        // 8. do head elims
        // your done!
        for snake_move in moves {
            match snake_move.move_type {
                MoveType::Death => {}
                MoveType::MoveSquare(square) => {
                    // 2.
                    out.tails.push((
                        snake_move.id,
                        self.board.snakes[snake_move.id].move_snake(square),
                    ));
                }
            }
        }
        for snake in &mut self.board.snakes {
            if snake.alive {
                snake.health -= 1;
            }
        }
        // feed snakes
        let mut eaten_food = 0;
        for snake in &mut self.board.snakes {
            if snake.alive && self.board.food & snake.body[0] == 1 {
                out.snake_eats.push((snake.id, snake.health));
                snake.feed();
                eaten_food |= snake.body[0];
            }
        }
        out.eaten_food = eaten_food;
        self.board.food ^= eaten_food;

        // out of bounds elims
        // can't happen thanks to movegen
        // health eliminations
        for snake in &mut self.board.snakes {
            if snake.health == 0 {
                out.kills.push(snake.id);
                snake.alive = false;
            }
        }

        // snake collision elims
        // specifically head to heads
        for snake in &self.board.snakes {
            if !snake.alive {
                continue;
            }

            for other in &self.board.snakes {
                if other.body[0] & snake.body[0] == 1 && other.body.len() >= snake.body.len() {
                    out.kills.push(snake.id);
                }
            }
        }
        for snake_move in moves {
            if MoveType::Death == snake_move.move_type {
                out.kills.push(snake_move.id);
            }
        }
        out.kills.sort();
        out.kills.dedup();

        for id in &out.kills {
            self.board.snakes[*id].alive = false;
        }

        out
    }
    pub fn is_terminal(&self) -> bool {
        // What defines a terminal state in battlesnake duels?
        // Num of alive snakes <= 1
        self.board.snakes.iter().filter(|x| x.alive).count() <= 1
    }

    pub fn hash(&self) -> u128 {
        let mut out = 0;
        for snake in &self.board.snakes {
            out |= snake.full;
        }
        out |= self.board.food;

        out
    }
}
