use super::Strategy;
use crate::COLS;

pub struct BackPropagationStrategy {
    pushed_columns: Vec<u8>,
}

impl BackPropagationStrategy {
    pub fn new() -> BackPropagationStrategy {
        BackPropagationStrategy {
            pushed_columns: vec![],
        }
    }
}

impl Strategy for BackPropagationStrategy {
    fn make_move(&mut self, board: &crate::board::Board) -> (u8, u8) {
        let mut temporary_game_field = board.clone();
        for col in 0..COLS {}
        (1, 1)
    }
}
