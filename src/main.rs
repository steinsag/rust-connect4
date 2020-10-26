use std::collections::HashMap;
use std::io;
use std::process;

use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};

const ROWS: u8 = 6;
const COLS: u8 = 7;
const COIN_HUMAN: char = 'H';
const COIN_AI: char = 'C';

#[derive(PartialEq)]
enum Winner {
    DRAW,
    HUMAN,
    AI,
}

#[derive(PartialEq)]
enum CellValue {
    EMPTY,
    HUMAN,
    AI,
}

struct Board {
    board: HashMap<u8, HashMap<u8, CellValue>>,
}

impl Board {
    pub fn new() -> Board {
        let mut board = HashMap::new();
        for col in 0..COLS {
            let mut col_map = HashMap::new();

            for row in 0..ROWS {
                col_map.insert(row, CellValue::EMPTY);
            }

            board.insert(col, col_map);
        }

        Board { board }
    }

    pub fn get_value(&self, col: &u8, row: &u8) -> &CellValue {
        &self.board[&col][&row]
    }

    fn find_lowest_empty_row(column: &HashMap<u8, CellValue>) -> Option<u8> {
        for row in 0..ROWS {
            let cell_value = column.get(&row).unwrap();
            if *cell_value == CellValue::EMPTY {
                return Some(row);
            }
        }
        return None;
    }

    pub fn get_lowest_empty_row(&self, col: &u8) -> Option<u8> {
        let column = self.board.get(col).unwrap();

        return Board::find_lowest_empty_row(&column);
    }

    pub fn add_coin(&mut self, col: &u8, coin: CellValue) {
        let col_map = self.board.get_mut(col).unwrap();
        match Board::find_lowest_empty_row(&col_map) {
            Some(row) => *col_map.entry(row).or_insert(CellValue::EMPTY) = coin,
            None => panic!("No empty row in column"),
        }
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn print_board(board: &Board) {
    println!("");
    for row in (0..ROWS).rev() {
        let mut line = format!("{} | ", row + 1);

        for col in 0..COLS {
            let cell_value = board.get_value(&col, &row);
            match cell_value {
                CellValue::EMPTY => line.push(' '),
                CellValue::AI => line.push(COIN_AI),
                _ => line.push(COIN_HUMAN),
            }
            if col != COLS - 1 {
                line.push(' ');
            }
        }
        println!("{}", line);
    }

    println!("   {}", "-".repeat((COLS * 2).into()));

    let mut col_legend = " ".repeat(4);
    for col in 0..COLS {
        col_legend.push_str(&format!("{} ", col + 1));
    }
    println!("{}", col_legend);
}

fn read_input(message: &str) -> String {
    let stdin = io::stdin();
    let input = &mut String::new();
    input.clear();

    println!("{} (enter \"q\" to exit game): ", message);

    stdin.read_line(input).expect("Could not read input.");
    let trimmed = input.trim();
    if trimmed == "q" {
        process::exit(0);
    }

    return trimmed.into();
}

fn read_column_input() -> u8 {
    loop {
        match (&read_input("Enter column")).parse::<u8>() {
            Ok(n) => {
                if n < 1 || n > COLS {
                    println!("Enter a number from 1 up to {} only.", COLS);
                    continue;
                } else {
                    return n - 1;
                }
            }
            Err(_) => {
                println!("Only enter single digits!");
                continue;
            }
        }
    }
}

fn make_human_move(board: &mut Board) {
    let col = read_column_input();

    match board.get_lowest_empty_row(&col) {
        Some(_) => board.add_coin(&col, CellValue::HUMAN),
        None => {
            println!("Move not allowed!");
            read_input("press return to continue");
        }
    }
}

fn make_ai_move(board: &mut Board, rng: &mut ThreadRng) {
    loop {
        let col: u8 = rng.gen_range(0, &COLS);
        match board.get_lowest_empty_row(&col) {
            Some(_) => {
                board.add_coin(&col, CellValue::AI);
                break;
            }
            None => continue,
        }
    }
}

fn check_winner(board: &Board) {
    // TODO
}

fn main() {
    let mut rng = thread_rng();
    let mut board = Board::new();
    loop {
        clear_screen();
        print_board(&board);

        make_human_move(&mut board);
        check_winner(&board);
        make_ai_move(&mut board, &mut rng);
        check_winner(&board);
    }
}
