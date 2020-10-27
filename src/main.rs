use std::cmp;
use std::collections::HashMap;
use std::io;
use std::process;

use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};

const ROWS: u8 = 6;
const COLS: u8 = 7;
const COIN_HUMAN: char = 'H';
const COIN_AI: char = 'C';

#[derive(PartialEq, Debug)]
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

    pub fn get_column(&self, col: &u8) -> &HashMap<u8, CellValue> {
        self.board.get(col).unwrap()
    }

    pub fn get_column_as_value_vector(&self, col: &u8) -> Vec<&CellValue> {
        let column = self.get_column(col);
        let mut value_vec = vec![];

        for row_idx in 0..ROWS {
            value_vec.push(column.get(&row_idx).unwrap());
        }

        return value_vec;
    }

    pub fn get_row_as_value_vector(&self, row: &u8) -> Vec<&CellValue> {
        let mut value_vec = vec![];

        for col_idx in 0..COLS {
            let column = self.get_column(&col_idx);

            value_vec.push(column.get(&row).unwrap());
        }

        return value_vec;
    }

    pub fn get_rising_diagonal_as_value_vector(&self, col: &u8, row: &u8) -> Vec<&CellValue> {
        let mut value_vec = vec![];

        let start_diff = cmp::min(col, row);
        let mut current_col = col - start_diff;
        let mut current_row = row - start_diff;

        while current_col < COLS && current_row < ROWS {
            value_vec.push(self.get_value(&current_col, &current_row));
            current_col += 1;
            current_row += 1;
        }

        return value_vec;
    }

    pub fn get_falling_diagonal_as_value_vector(&self, col: &u8, row: &u8) -> Vec<&CellValue> {
        let mut value_vec = vec![];

        let row_diff = ROWS - row - 1;

        let start_diff = cmp::min(col, &row_diff);

        let mut current_col = col - start_diff;
        let mut current_row = row + start_diff;

        loop {
            value_vec.push(self.get_value(&current_col, &current_row));
            if current_col == COLS - 1 || current_row == 0 {
                break;
            }
            current_col += 1;
            current_row -= 1;
        }

        return value_vec;
    }

    pub fn get_lowest_empty_row(&self, col: &u8) -> Option<u8> {
        Board::find_lowest_empty_row(self.get_column(col))
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

fn make_human_move(board: &mut Board) -> Option<(u8, u8)> {
    let col = read_column_input();

    match board.get_lowest_empty_row(&col) {
        Some(row) => {
            board.add_coin(&col, CellValue::HUMAN);
            return Some((col, row));
        }
        None => {
            println!("Move not allowed!");
            read_input("press return to continue");
            return None;
        }
    }
}

fn make_ai_move(board: &mut Board, rng: &mut ThreadRng) -> Option<(u8, u8)> {
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

fn value_vector_to_bits(value_vector: &Vec<&CellValue>, mask_value: &CellValue) -> u8 {
    let mut bits = 0;
    let mut idx = 0;
    let vector_len = value_vector.len();

    for value in value_vector {
        if *value == mask_value {
            bits += 2 << (vector_len - idx - 1);
        }

        idx += 1;
    }

    return bits;
}

fn has_winning_bits(bits: &u8, cell_value: &CellValue) {
    if bits & 15 == 15
        || bits & 30 == 30
        || bits & 60 == 60
        || bits & 120 == 120
        || bits & 240 == 240
    {
        if *cell_value == CellValue::HUMAN {
            println!("Human wins!");
        } else {
            println!("Computer wins!");
        }
        process::exit(0);
    }
}

fn check_winner(board: &Board, col: &u8, row: &u8) {
    let cell_value = board.get_value(col, row);

    if *cell_value == CellValue::EMPTY {
        panic!("Last entered value should not be EMPTY.");
    }

    let column_bits = value_vector_to_bits(&board.get_column_as_value_vector(col), cell_value);
    has_winning_bits(&column_bits, cell_value);

    let row_bits = value_vector_to_bits(&board.get_row_as_value_vector(row), cell_value);
    has_winning_bits(&row_bits, cell_value);

    let diagonal_inc_bits = value_vector_to_bits(
        &board.get_rising_diagonal_as_value_vector(col, row),
        cell_value,
    );
    has_winning_bits(&diagonal_inc_bits, cell_value);

    let diagonal_decr_bits = value_vector_to_bits(
        &board.get_falling_diagonal_as_value_vector(col, row),
        cell_value,
    );
    has_winning_bits(&diagonal_decr_bits, cell_value);
}

fn main() {
    let mut rng = thread_rng();
    let mut board = Board::new();
    loop {
        clear_screen();
        print_board(&board);

        match make_human_move(&mut board) {
            Some((col, row)) => {
                clear_screen();
                print_board(&board);
                check_winner(&board, &col, &row);
            }
            None => continue,
        }

        let (col, row) = make_ai_move(&mut board, &mut rng).unwrap();
        clear_screen();
        print_board(&board);
        check_winner(&board, &col, &row);
    }
}
