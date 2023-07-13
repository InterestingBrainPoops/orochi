use crate::{
    useful_board::{Game, Side, Snake},
    UNIVERSE,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub move_type: MoveType,
    pub id: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]

pub enum MoveType {
    Death,
    MoveSquare(u128),
}

impl Move {
    pub fn new_square(end_square: u128, id: usize) -> Self {
        Self {
            move_type: MoveType::MoveSquare(end_square),
            id,
        }
    }
    pub fn new_death(id: usize) -> Self {
        Self {
            move_type: MoveType::Death,
            id,
        }
    }
}

impl Game {
    pub fn get_current_side_moves(&self) -> Vec<Move> {
        let mut all_things = 0;
        for snake in &self.board.snakes {
            let mut full = snake.full;
            if snake.body[snake.body.len() - 1] != snake.body[snake.body.len() - 2] {
                full ^= snake.body[snake.body.len() - 1];
            }
            all_things |= full ^ snake.body[0];
        }
        let snake_id = if self.side == Side::You {
            self.you_id
        } else if self.you_id == 1 {
            0
        } else {
            1
        };
        let mut snake_moves = self.board.snakes[snake_id].moves() & !all_things;
        let mut out = vec![];

        assert!(snake_moves.count_ones() < 4);
        if snake_moves.count_ones() == 0 {
            return vec![Move::new_death(snake_id)];
        }
        for _ in 0..snake_moves.count_ones() {
            let end_square = 1 << snake_moves.trailing_zeros();
            snake_moves ^= end_square;

            out.push(Move::new_square(end_square, snake_id))
        }

        out
    }
}

impl Snake {
    pub fn moves(&self) -> u128 {
        ((self.body[0] << 1 & UNIVERSE) & !0x4008010020040080100200400801)
            | ((self.body[0] >> 1) & !0x1002004008010020040080100200400)
            | (self.body[0] >> 11)
            | ((self.body[0] << 11) & UNIVERSE)
    }
}
