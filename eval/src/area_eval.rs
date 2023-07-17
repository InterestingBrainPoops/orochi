use board::{
    useful_board::{Board, Game, Snake},
    Coordinate,
};
use nalgebra::SVector;
use pathfinding::prelude::astar;
use snake_tuner::{
    activation::functions::Sigmoid,
    evaluation::{evaluations::Linear, Eval},
};

pub struct AreaEval {
    pub eval: Linear<5, Sigmoid>,
}
impl AreaEval {
    pub fn score(&self, position: &Game) -> f64 {
        self.eval.forward(Self::score_i(position))
    }
    fn score_i(position: &Game) -> SVector<f64, 5> {
        // me
        let me = position.board.snakes[position.you_id].clone();
        // them
        let other_id = if position.you_id == 0 { 1 } else { 0 };
        let other = position.board.snakes[other_id].clone();
        // the length difference between me and them
        let length_difference = me.body.len() as i32 - other.body.len() as i32;
        // my distance to center - their distance to center
        let distance_to_center = manhattan(&me.body[0].into(), &Coordinate::new(6, 6))
            - manhattan(&other.body[0].into(), &Coordinate::new(6, 6));
        // my heatlh - their health
        let health_diff = me.health as i32 - other.health as i32;
        // my nearest food
        let mut my_nearest = 0;
        // their nearest food
        let mut their_nearest = 0;
        let mut food_holder = position.board.food;
        while food_holder.count_ones() != 0 {
            let food = 1 << food_holder.trailing_zeros();
            food_holder ^= food;
            // my path to the food
            let my_path = astar(
                &me.body[0].into(),
                |p| successors(p, &position.board),
                |p| manhattan(p, &Coordinate::from(food)),
                |p| *p == Coordinate::from(food),
            );
            // their path to the same food
            let their_path = astar(
                &other.body[0].into(),
                |p| successors(p, &position.board),
                |p| manhattan(p, &Coordinate::from(food)),
                |p| *p == Coordinate::from(food),
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
        for x in 0..11 {
            for y in 0..11 {
                // the curent coordinate
                let thing = &Coordinate::new(x, y);
                // if the square is in either persons body, ignore it
                if other.body.contains(&thing.into_mask(11))
                    || me.body.contains(&thing.into_mask(11))
                {
                    continue;
                }

                // my path to the square
                let my_path = astar(
                    &me.body[0].into(),
                    |p| successors(p, &position.board),
                    |p| manhattan(p, thing),
                    |p| *p == *thing,
                );

                // their path to the square
                let their_path = astar(
                    &other.body[0].into(),
                    |p| successors(p, &position.board),
                    |p| manhattan(p, thing),
                    |p| *p == *thing,
                );
                // my distance to the square
                let my_dist = match my_path {
                    None => 1000,                  // if i dont have a path, set it to 1k
                    Some((path, _)) => path.len(), // if i do, set it to the length of the path
                };
                let their_dist = match their_path {
                    None => 1000,                  // if they dont have a path, set it to 1k
                    Some((path, _)) => path.len(), // if they do, set to the length of the path
                };
                // credit based on who is closer
                if my_dist < their_dist {
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
    let possible = Snake::square_moves(coord.into_mask(11));
    let mut all_things = 0;
    for snake in &board.snakes {
        let mut full = snake.full;
        if snake.body[snake.body.len() - 1] != snake.body[snake.body.len() - 2] {
            full ^= snake.body[snake.body.len() - 1];
        }
        all_things |= full ^ snake.body[0];
    }
    let mut out = vec![];
    let mut possible = possible & !all_things;

    while possible.count_ones() != 0 {
        let square = 1 << possible.trailing_zeros();
        possible ^= square;
        out.push(square.into());
    }
    // add a weight to each one
    out.iter().map(|p| (*p, 1)).collect()
}

// manhattan distance between two coordinates
fn manhattan(c1: &Coordinate, c2: &Coordinate) -> i32 {
    (c1.x - c2.x).abs() as i32 + (c1.y - c2.y).abs() as i32
}
