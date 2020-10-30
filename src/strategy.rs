mod back_propagation;
mod random;

use super::board::Board;

pub trait Strategy {
    fn make_move(&mut self, board: &Board) -> (u8, u8);
}

pub fn select_strategy(strategy_id: &str) -> Box<dyn Strategy> {
    match strategy_id {
        "back" => return Box::new(back_propagation::BackPropagationStrategy::new()),
        _ => return Box::new(random::RandomStrategy::new()),
    }
}
