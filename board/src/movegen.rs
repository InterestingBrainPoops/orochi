use crate::{
    useful_board::{Game, Side, Snake},
    UNIVERSE,
};

#[derive(Clone, Copy)]
pub struct Move {
    pub end_square: u128,
    pub id: usize,
}

impl Move {
    pub fn new(end_square: u128, id: usize) -> Self {
        Self { end_square, id }
    }
}

impl Game {
    pub fn get_current_side_moves(&self) -> Vec<Move> {
        let mut all_things = 0;
        for snake in &self.board.snakes {
            all_things |= snake.full;
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

        for _ in 0..snake_moves.count_ones() {
            let end_square = 1 << snake_moves.trailing_zeros();
            snake_moves ^= end_square;

            out.push(Move {
                end_square,
                id: snake_id,
            })
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
