use std::ops::{Index, IndexMut};

use board::{movegen::Move, useful_board::Game};
use eval::Eval;

struct MoveTable {
    inner: Vec<TableEntry>,
}

impl Index<&Game> for MoveTable {
    type Output = TableEntry;

    fn index(&self, index: &Game) -> &Self::Output {
        &self.inner[index.hash() as usize % self.inner.len()]
    }
}

impl IndexMut<&Game> for MoveTable {
    fn index_mut(&mut self, index: &Game) -> &mut Self::Output {
        let size = self.inner.len();
        &mut self.inner[index.hash() as usize % size]
    }
}

#[derive(Clone)]
struct TableEntry {
    you: Option<Move>,
    them: Option<Move>,
}

impl TableEntry {
    fn new() -> Self {
        Self {
            you: None,
            them: None,
        }
    }
}

pub struct Search<E: Eval> {
    move_table: MoveTable,
    eval: E,
    pub statistics: Stats,
}

#[derive(Clone, Copy)]
pub struct Stats {
    pub node_count: u64,
    pub tt_hits: u64,
}
impl<E: Eval> Search<E> {
    pub fn reset(&mut self) {
        self.move_table.inner.fill(TableEntry::new())
    }
    pub fn new(table_size: usize, eval: E) -> Search<E> {
        let mut table = vec![];

        table.resize(table_size, TableEntry::new());

        Search {
            move_table: MoveTable { inner: table },
            eval,
            statistics: Stats {
                node_count: 0,
                tt_hits: 0,
            },
        }
    }
    fn update_move(&mut self, board: &Game, you: bool, mov: Move) {
        let entry = &mut self.move_table[board];
        let move_ref = if you { &mut entry.you } else { &mut entry.them };
        if let Some(x) = move_ref {
            *x = mov;
        } else {
            *move_ref = Some(mov);
        }
    }
    pub fn iterative_deepen(&mut self, board: &mut Game, target_depth: u32) -> (Move, f64) {
        let mut score = 0.0;

        for depth in 1..(target_depth + 1) {
            score = self.minimax(board, depth, -100.0, 100.0, true, None);
        }
        let entry = self.move_table[board].clone();

        (entry.you.unwrap(), score)
    }
    fn sort_moves(&mut self, board: &Game, you: bool) -> Vec<Move> {
        let mut out = board.get_current_side_moves(you);
        let entry = self.move_table[board].clone();
        if let Some(hash_move) = if you { entry.you } else { entry.them } {
            if let Some(old_idx) = out.iter().position(|x| *x == hash_move) {
                self.statistics.tt_hits += 1;
                out.swap(0, old_idx)
            }
        }
        out
    }
    fn minimax(
        &mut self,
        board: &mut Game,
        depth: u32,
        mut alpha: f64,
        mut beta: f64,
        maximizing: bool,
        you_move: Option<Move>,
    ) -> f64 {
        if board.is_terminal() {
            return 0.0;
        } else if depth == 0 {
            return self.eval.get_valuation(board);
        }
        if maximizing {
            for you_move in &self.sort_moves(board, maximizing) {
                self.statistics.node_count += 1;

                let score = self.minimax(board, depth, alpha, beta, !maximizing, Some(*you_move));

                if score > alpha {
                    self.update_move(board, maximizing, *you_move);
                    alpha = score;
                }
                if score >= beta {
                    self.update_move(board, maximizing, *you_move);
                    return beta;
                }
            }
            alpha
        } else {
            for moves in self
                .sort_moves(board, maximizing)
                .iter()
                .map(|x| vec![*x, you_move.unwrap()])
            {
                // let eboard = board.clone();
                let undo = board.step(&moves);
                self.statistics.node_count += 1;
                let score = self.minimax(board, depth - 1, alpha, beta, !maximizing, None);
                board.undo(&undo);
                // if eboard != *board {
                //     println!("{depth}");
                //     println!("{:?}, {:?}", moves, undo);

                //     assert_eq!(eboard, *board);
                // }

                if score < beta {
                    self.update_move(board, maximizing, moves[0]);
                    beta = score;
                }

                if score <= alpha {
                    self.update_move(board, maximizing, moves[0]);
                    return alpha;
                }
            }
            beta
        }
    }
}
