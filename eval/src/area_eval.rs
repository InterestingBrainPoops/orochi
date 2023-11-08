use board::{
    useful_board::{Board, Game, Snake},
    Coordinate,
};
use nalgebra::SVector;
use pathfinding::prelude::astar;
use snake_tuner::{
    activation::{functions::Sigmoid, ActivationFunction},
    evaluation::{evaluations::Linear, Eval},
};

#[derive(Clone)]
pub struct AreaEval {
    pub eval: Linear<5, Sigmoid>,
}

impl crate::Eval for AreaEval {
    fn get_valuation(&self, game: &Game) -> f64 {
        self.score(game)
    }
}

impl AreaEval {
    pub fn new(weights: [f64; 5]) -> AreaEval {
        AreaEval {
            eval: Linear::from_weights(SVector::from(weights), Sigmoid),
        }
    }
    pub fn score(&self, position: &Game) -> f64 {
        let x = Sigmoid;
        x.evaluate(self.eval.forward(Self::label(position)))
    }
    fn label(position: &Game) -> SVector<f64, 5> {
        assert!(position.board.snakes.len() == 2);
        // me
        let me = &position.board.snakes[position
            .board
            .snakes
            .iter()
            .position(|x| &x.id == &position.you_id)
            .unwrap()];
        // them
        let other = &position.board.snakes[position
            .board
            .snakes
            .iter()
            .position(|x| x.id != position.you_id)
            .unwrap()];
        // the length difference between me and them
        let length_difference = me.body.len() as i32 - other.body.len() as i32;
        // my distance to center - their distance to center
        let center = Coordinate::new(6, 6);
        let distance_to_center =
            manhattan(&me.body[0], &center) - manhattan(&other.body[0], &center);
        // my heatlh - their health
        let health_diff = me.health as i32 - other.health as i32;
        // my nearest food
        let mut my_nearest = 0;
        // their nearest food
        let mut their_nearest = 0;
        for food in &position.board.food {
            // my path to the food
            let my_path = astar(
                &me.body[0],
                |p| successors(p, &position.board),
                |p| manhattan(p, &food),
                |p| p == food,
            );
            // their path to the same food
            let their_path = astar(
                &other.body[0],
                |p| successors(p, &position.board),
                |p| manhattan(p, &food),
                |p| p == food,
            );
            // my distance to the food
            let my_dist = match my_path {
                None => 1000,                  // if i have no path, set the path length to 1k
                Some((path, _)) => path.len(), // otherwise set it to the length of the path
            };
            // their distance to the food
            let their_dist = match their_path {
                None => 1000,                  // if they have no path, set the path length to 1k
                Some((path, _)) => path.len(), // otherwise set it to the length of the path
            };
            // give credit based on whose path is shorter
            if my_dist < their_dist {
                // if my path is shorter, then credit me
                my_nearest += 1;
            } else {
                // if their path is shorter, then credit them
                their_nearest += 1;
            }
        }
        // my_nearest foods - their_nearest-foods
        let food_ownership_difference = my_nearest - their_nearest;
        // my owned squares
        let mut my_squares = 0;
        // their owned squares
        let mut their_squares = 0;
        // go through all the squares on the board
        let mut paths = [[None; 11]; 11];
        for x in 0..11 {
            for y in 0..11 {
                if paths[x as usize][y as usize].is_some() {
                    continue;
                }
                // the curent coordinate
                let thing = &Coordinate::new(x, y);
                // if the square is in either persons body, ignore it
                if position
                    .board
                    .snakes
                    .iter()
                    .any(|snake| snake.body.contains(thing))
                {
                    continue;
                }

                // my path to the square
                let my_path = astar(
                    &me.body[0],
                    |p| successors(p, &position.board),
                    |p| manhattan(p, thing),
                    |p| *p == *thing,
                );

                // their path to the square
                let their_path = astar(
                    &other.body[0],
                    |p| successors(p, &position.board),
                    |p| manhattan(p, thing),
                    |p| *p == *thing,
                );

                if let (Some((path, _)), Some((path2, _))) = (&my_path, &their_path) {
                    for (idx, coord) in path.iter().enumerate() {
                        if !path2.contains(coord) {
                            paths[coord.x as usize][coord.y as usize] = Some(true);
                            continue;
                        }
                        // path 2 does contain x as well as me
                        // so if my remaining length is shorter than his, then I am closer.
                        if path2.len() - path2.iter().position(|y| coord == y).unwrap()
                            > path.len() - idx
                        {
                            paths[coord.x as usize][coord.y as usize] = Some(true);
                        }
                    }

                    for (idx, coord) in path2.iter().enumerate() {
                        if !path.contains(coord) {
                            paths[coord.x as usize][coord.y as usize] = Some(false);
                            continue;
                        }
                        // path 2 does contain x as well as me
                        // so if my remaining length is shorter than his, then I am closer.
                        if path.len() - path.iter().position(|y| coord == y).unwrap()
                            > path2.len() - idx
                        {
                            paths[coord.x as usize][coord.y as usize] = Some(false);
                        }
                    }
                }
            }
        }
        for row in &paths {
            for square in row.iter().flatten() {
                if *square {
                    my_squares += 1;
                } else {
                    their_squares += 1;
                }
            }
        }
        // the difference between the owned squares
        let square_ownership_difference = my_squares - their_squares;

        SVector::from([
            length_difference as f64,
            distance_to_center as f64,
            health_diff as f64,
            food_ownership_difference as f64,
            square_ownership_difference as f64,
        ])
    }
}
// this is always from the perspective of the first snake (hacky fix, but it works)

// successors for a given coordinate
fn successors(coord: &Coordinate, board: &Board) -> Vec<(Coordinate, i32)> {
    // possible successors
    let possible = Snake::square_moves(*coord);
    // add a weight to each one
    possible
        .iter()
        .filter(|&&square| {
            square.x >= 0
                && square.x < board.width as i8
                && square.y > 0
                && square.y < board.height as i8
                && board
                    .snakes
                    .iter()
                    .all(|x| x.body.iter().all(|&x| x != square))
        })
        .map(|&square| (square, 1))
        .collect()
}

// manhattan distance between two coordinates
fn manhattan(c1: &Coordinate, c2: &Coordinate) -> i32 {
    (c1.x - c2.x).abs() as i32 + (c1.y - c2.y).abs() as i32
}
