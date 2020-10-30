use std::env;

const ROWS: u8 = 6;
const COLS: u8 = 7;

mod board;
mod game;
mod strategy;
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

fn main() {
    let mut board = board::Board::new();

    let args: Vec<String> = env::args().collect();
    let strategy_id = if args.len() == 2 { &args[1] } else { "random" };
    let mut ai = strategy::select_strategy(strategy_id);

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

        let (col, row) = ai.make_move(&board);
        board.add_coin(&col, CellValue::AI);
        ui::clear_screen();
        board.print();
        game::complete_game_if_finished(&game::get_winner(&board, &col, &row));
    }
}
