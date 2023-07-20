use board::{movegen::Move, useful_board::Game};
use eval::Eval;
use rand::{rngs::ThreadRng, seq::IteratorRandom, thread_rng};

#[derive(Debug)]
struct IdxRange {
    start: usize,
    len: usize,
}

impl IdxRange {
    fn new(start: usize, len: usize) -> Self {
        Self { start, len }
    }
}
#[derive(Debug)]
struct Node {
    move_to_here: Move,
    num_simulations: u32,
    score_total: f64,
    parent: Option<usize>,
    children: IdxRange,
}
impl Node {
    pub fn new(parent: Option<usize>, move_to_here: Move) -> Self {
        Node {
            move_to_here,
            num_simulations: 0,
            score_total: 0.0,
            children: IdxRange::new(0, 0),
            parent,
        }
    }
}
pub struct Search<E: Eval> {
    start: Game,
    eval: E,
    rng: ThreadRng,
    tree: Vec<Node>,
}

impl<E: Eval> Search<E> {
    pub fn root_score(&self) -> f64 {
        self.tree[0].score_total / self.tree[0].num_simulations as f64
    }
    pub fn new(game: Game, eval: E) -> Search<E> {
        Search {
            tree: vec![Node::new(None, Move::new_death(0))],
            start: game,
            eval,
            rng: thread_rng(),
        }
    }

    pub fn iterate(&mut self) {
        let mut game = self.start.clone();
        // println!("root {:?}", self.tree[0]);

        let next_leaf = self.next_leaf_node(&mut game);

        // println!("{}", next_leaf);
        let score = self.score(&game);
        if (0.5 - score).abs() > 0.3 {
            println!("e");
        }
        self.backprop(next_leaf, score);
    }

    fn score(&self, game: &Game) -> f64 {
        match game.side {
            board::useful_board::Side::You => self.eval.get_valuation(game),
            board::useful_board::Side::Them => 1.0 - self.eval.get_valuation(game),
        }
    }
    fn next_leaf_node(&mut self, game: &mut Game) -> usize {
        let mut best_node = 0;
        while self.tree[best_node].expanded() {
            best_node = self.pick_child(game, best_node);
        }
        // println!("cee{best_node}");
        self.expand(best_node, game);
        self.pick_random(best_node, game)
    }

    fn expand(&mut self, node: usize, game: &Game) {
        if game.is_terminal() {
            return;
        }
        let moves = game.get_current_side_moves();
        self.tree[node].children = IdxRange::new(self.tree.len(), moves.len());
        for x in &moves {
            self.tree.push(Node::new(Some(node), *x))
        }
    }
    fn pick_random(&mut self, node: usize, game: &mut Game) -> usize {
        let range = &self.tree[node].children;
        let idx = (range.start..(range.start + range.len))
            .choose(&mut self.rng)
            .unwrap();
        game.step(&self.tree[idx].move_to_here);
        idx
    }
    fn pick_child(&self, game: &mut Game, best_node: usize) -> usize {
        let node = &self.tree[best_node];
        // dbg!(best_node);
        // dbg!((node.children.start..(node.children.start + node.children.len)));
        let highest_index = (node.children.start..(node.children.start + node.children.len))
            .max_by(|&a, &b| {
                node.uct(&self.tree[a])
                    .partial_cmp(&node.uct(&self.tree[b]))
                    .unwrap()
            })
            .unwrap();
        // the chosen child based on UCT score
        game.step(&self.tree[highest_index].move_to_here);
        highest_index
    }
    fn backprop(&mut self, leaf: usize, mut score: f64) {
        let mut current_node = leaf;
        while let Some(node_idx) = self.tree[current_node].parent {
            let node = &mut self.tree[current_node];
            node.score_total += score;
            node.num_simulations += 1;
            score = 1.0 - score;
            current_node = node_idx;
        }

        let node = &mut self.tree[current_node];
        node.score_total += score;
        node.num_simulations += 1;
    }
}
impl Node {
    fn expanded(&self) -> bool {
        self.children.len != 0
    }

    fn uct(&self, child: &Node) -> f64 {
        let value = (child.score_total / child.num_simulations as f64)
            + 2f64.sqrt()
                * (f64::ln(self.num_simulations as f64) / child.num_simulations as f64).sqrt();
        if value.is_nan() {
            0.0
        } else {
            value
        }
    }
}
