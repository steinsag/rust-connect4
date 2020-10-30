use super::Strategy;
use crate::COLS;

use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};

pub struct RandomStrategy {
    rng: ThreadRng,
}

impl RandomStrategy {
    pub fn new() -> RandomStrategy {
        RandomStrategy { rng: thread_rng() }
    }
}

impl Strategy for RandomStrategy {
    fn make_move(&mut self, board: &crate::board::Board) -> (u8, u8) {
        loop {
            let col = self.rng.gen_range(0, &COLS);
            match board.get_lowest_empty_row(&col) {
                Some(row) => return (col, row),
                None => continue,
            }
        }
    }
}
