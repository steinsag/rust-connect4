use std::cmp;

use crate::{COLS, ROWS};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CellValue {
    EMPTY,
    HUMAN,
    AI,
}

impl CellValue {
    pub fn coin_value(&self) -> char {
        match *self {
            CellValue::EMPTY => ' ',
            CellValue::AI => 'C',
            CellValue::HUMAN => 'H',
        }
    }
}

pub struct Board {
    // columns from left to right
    // rows from bottom to top
    // columns of bottom row, ..., columns of top row
    board: Vec<CellValue>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: (0..(ROWS * COLS)).map(|_| CellValue::EMPTY).collect(),
        }
    }

    #[cfg(test)]
    pub fn from(rows: Vec<Vec<CellValue>>) -> Board {
        let mut board = Board::new();
        let mut row_idx: u8 = 0;
        for row in rows.iter().rev() {
            let mut col_idx = 0;

            for cell_value in row {
                if *cell_value == CellValue::EMPTY {
                    col_idx += 1;
                    continue;
                }

                board.set(&col_idx, &row_idx, *cell_value);
                col_idx += 1;
            }

            row_idx += 1;
        }

        return board;
    }

    pub fn get(&self, col: &u8, row: &u8) -> &CellValue {
        &self.board[(row * COLS + col) as usize]
    }

    fn set(&mut self, col: &u8, row: &u8, value: CellValue) {
        self.board[(row * COLS + col) as usize] = value;
    }

    fn find_lowest_empty_row(column: Vec<&CellValue>) -> Option<u8> {
        let mut row_idx = 0;

        for cell_value in column {
            if *cell_value == CellValue::EMPTY {
                return Some(row_idx);
            }
            row_idx += 1;
        }
        return None;
    }

    pub fn get_column(&self, col: &u8) -> Vec<&CellValue> {
        let mut column = vec![];

        for row in 0..ROWS {
            column.push(self.get(col, &row));
        }
        return column;
    }

    pub fn get_row(&self, row: &u8) -> Vec<&CellValue> {
        let mut row_vec = vec![];
        for col in 0..COLS {
            row_vec.push(self.get(&col, row));
        }
        return row_vec;
    }

    pub fn get_rising_diagonal_as_value_vector(&self, col: &u8, row: &u8) -> Vec<&CellValue> {
        let mut value_vec = vec![];

        let start_diff = cmp::min(col, row);
        let mut current_col = col - start_diff;
        let mut current_row = row - start_diff;

        while current_col < COLS && current_row < ROWS {
            value_vec.push(self.get(&current_col, &current_row));
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
            value_vec.push(self.get(&current_col, &current_row));
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
        let column = self.get_column(col);
        match Board::find_lowest_empty_row(column) {
            Some(row) => self.set(col, &row, coin),
            None => panic!("No empty row in column"),
        }
    }

    pub fn print(&self) {
        println!("");
        for row in (0..ROWS).rev() {
            let mut line = format!("{} | ", row + 1);
            for col in 0..COLS {
                line.push(self.get(&col, &row).coin_value());
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
}

#[cfg(test)]
mod tests {
    use super::{Board, CellValue, COLS, ROWS};
    #[test]
    fn test_new() {
        let board = Board::new();
        assert_eq!(COLS * ROWS, board.board.len() as u8);

        for col_idx in 0..COLS {
            let column = board.get_column(&col_idx);
            assert_eq!(ROWS, column.len() as u8);

            for row_idx in 0..ROWS {
                let cell_value = column[row_idx as usize];
                assert_eq!(CellValue::EMPTY, *cell_value);
            }
        }
    }

    #[test]
    fn test_from() {
        use CellValue::{AI as C, EMPTY as E, HUMAN as H};
        let board = Board::from(vec![
            vec![E, E, E, E, E, E, H],
            vec![E, E, E, E, E, C, C],
            vec![E, E, E, E, H, H, H],
            vec![E, E, E, C, C, C, C],
            vec![E, E, H, H, H, H, H],
            vec![E, C, C, C, C, C, C],
        ]);

        assert_eq!(&E, board.get(&0, &0));
        assert_eq!(&E, board.get(&0, &5));
        assert_eq!(&C, board.get(&6, &0));
        assert_eq!(&H, board.get(&6, &5));
    }

    #[test]
    fn test_find_lowest_empty_row() {
        use CellValue::{AI as C, EMPTY as E, HUMAN as H};
        let empty_board = Board::new();

        for col_idx in 0..COLS {
            assert_eq!(empty_board.get_lowest_empty_row(&col_idx).unwrap(), 0);
        }

        let board = Board::from(vec![
            vec![E, E, E, E, E, E, H],
            vec![E, E, E, E, E, C, C],
            vec![E, E, E, E, H, H, H],
            vec![E, E, E, C, C, C, C],
            vec![E, E, H, H, H, H, H],
            vec![E, C, C, C, C, C, C],
        ]);

        assert_eq!(0, board.get_lowest_empty_row(&0).unwrap());
        assert_eq!(1, board.get_lowest_empty_row(&1).unwrap());
        assert_eq!(2, board.get_lowest_empty_row(&2).unwrap());
        assert_eq!(3, board.get_lowest_empty_row(&3).unwrap());
        assert_eq!(4, board.get_lowest_empty_row(&4).unwrap());
        assert_eq!(5, board.get_lowest_empty_row(&5).unwrap());
        assert_eq!(None, board.get_lowest_empty_row(&6));
    }
}
