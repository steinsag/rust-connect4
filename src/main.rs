use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};

const ROWS: u8 = 6;
const COLS: u8 = 7;

mod board;
mod game;
mod ui;

use board::CellValue;

fn make_human_move(board: &mut board::Board) -> Option<(u8, u8)> {
    let col = ui::read_column_input();

    match board.get_lowest_empty_row(&col) {
        Some(row) => {
            board.add_coin(&col, CellValue::HUMAN);
            return Some((col, row));
        }
        None => {
            println!("Move not allowed!");
            ui::read_input("press return to continue");
            return None;
        }
    }
}

fn make_ai_move(board: &mut board::Board, rng: &mut ThreadRng) -> Option<(u8, u8)> {
    loop {
        let col: u8 = rng.gen_range(0, &COLS);
        match board.get_lowest_empty_row(&col) {
            Some(row) => {
                board.add_coin(&col, CellValue::AI);
                return Some((col, row));
            }
            None => continue,
        }
    }
}

fn main() {
    let mut rng = thread_rng();
    let mut board = board::Board::new();

    loop {
        ui::clear_screen();
        board.print();

        match make_human_move(&mut board) {
            Some((col, row)) => {
                ui::clear_screen();
                board.print();
                game::complete_game_if_finished(&game::get_winner(&board, &col, &row));
            }
            None => continue,
        }

        let (col, row) = make_ai_move(&mut board, &mut rng).unwrap();
        ui::clear_screen();
        board.print();
        game::complete_game_if_finished(&game::get_winner(&board, &col, &row));
    }
}
