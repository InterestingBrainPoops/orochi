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
        let mut out = (None, 0.0);

        for depth in 1..(target_depth + 1) {
            out = self.minimax(board, depth, -100.0, 100.0, true, None);
        }
        println!("{out:?}");
        (out.0.unwrap(), out.1)
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
    ) -> (Option<Move>, f64) {
        if board.is_terminal() {
            if board.board.snakes[board.you_id].alive {
                return (None, 1.0);
            } else if board.board.snakes.iter().filter(|x| x.alive).count() == 1 {
                return (None, 0.0);
            } else {
                return (None, 0.5);
            }
        } else if depth == 0 {
            return (None, self.eval.get_valuation(board));
        }
        if maximizing {
            let mut best_score: f64 = -100.0;
            let mut best_move = None;
            for you_move in &self.sort_moves(board, maximizing) {
                if best_move.is_none() {
                    best_move = Some(*you_move);
                }
                self.statistics.node_count += 1;
                // PVS full window first
                let (_, score) =
                    self.minimax(board, depth, alpha, beta, !maximizing, Some(*you_move));
                if score > best_score {
                    best_score = score;
                    best_move = Some(*you_move);
                }
                if best_score > alpha {
                    alpha = best_score;
                }
                if best_score >= beta {
                    self.update_move(board, maximizing, *you_move);
                    return (Some(*you_move), beta);
                }
            }
            self.update_move(board, maximizing, best_move.unwrap());
            (best_move, best_score)
        } else {
            let mut best_score: f64 = 100.0;
            for moves in self
                .sort_moves(board, maximizing)
                .iter()
                .map(|x| vec![*x, you_move.unwrap()])
            {
                // let eboard = board.clone();
                let undo = board.step(&moves);
                self.statistics.node_count += 1;
                let (_, score) = self.minimax(board, depth - 1, alpha, beta, !maximizing, None);
                board.undo(&undo);
                best_score = best_score.min(score);
                if best_score < beta {
                    self.update_move(board, maximizing, moves[0]);
                    beta = score;
                }

                if best_score <= alpha {
                    self.update_move(board, maximizing, moves[0]);
                    return (None, alpha);
                }
            }
            (None, best_score)
        }
    }
}
