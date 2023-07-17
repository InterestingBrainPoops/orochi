pub mod area_eval;

use board::useful_board::Game;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub trait Eval {
    fn get_valuation(&self, game: &Game) -> f64;
}
