use std::cmp;
use std::collections::HashMap;

const COIN_HUMAN: char = 'H';
const COIN_AI: char = 'C';

#[derive(PartialEq, Debug)]
pub enum CellValue {
    EMPTY,
    HUMAN,
    AI,
}

pub struct Board {
    board: HashMap<u8, HashMap<u8, CellValue>>,
}

impl Board {
    pub fn new() -> Board {
        let mut board = HashMap::new();
        for col in 0..super::COLS {
            let mut col_map = HashMap::new();

            for row in 0..super::ROWS {
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
        for row in 0..super::ROWS {
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

        for row_idx in 0..super::ROWS {
            value_vec.push(column.get(&row_idx).unwrap());
        }

        return value_vec;
    }

    pub fn get_row_as_value_vector(&self, row: &u8) -> Vec<&CellValue> {
        let mut value_vec = vec![];

        for col_idx in 0..super::COLS {
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

        while current_col < super::COLS && current_row < super::ROWS {
            value_vec.push(self.get_value(&current_col, &current_row));
            current_col += 1;
            current_row += 1;
        }

        return value_vec;
    }

    pub fn get_falling_diagonal_as_value_vector(&self, col: &u8, row: &u8) -> Vec<&CellValue> {
        let mut value_vec = vec![];

        let row_diff = super::ROWS - row - 1;

        let start_diff = cmp::min(col, &row_diff);

        let mut current_col = col - start_diff;
        let mut current_row = row + start_diff;

        loop {
            value_vec.push(self.get_value(&current_col, &current_row));
            if current_col == super::COLS - 1 || current_row == 0 {
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

    pub fn print(&self) {
        println!("");
        for row in (0..super::ROWS).rev() {
            let mut line = format!("{} | ", row + 1);
            for col in 0..super::COLS {
                let cell_value = self.get_value(&col, &row);
                match cell_value {
                    CellValue::EMPTY => line.push(' '),
                    CellValue::AI => line.push(COIN_AI),
                    _ => line.push(COIN_HUMAN),
                }
                if col != super::COLS - 1 {
                    line.push(' ');
                }
            }
            println!("{}", line);
        }
        println!("   {}", "-".repeat((super::COLS * 2).into()));
        let mut col_legend = " ".repeat(4);
        for col in 0..super::COLS {
            col_legend.push_str(&format!("{} ", col + 1));
        }
        println!("{}", col_legend);
    }
}
