use std::process;

#[derive(PartialEq)]
pub enum Winner {
    HUMAN,
    AI,
    DRAFT,
    NOBODY,
}

fn value_vector_to_bits(
    value_vector: &Vec<&super::board::CellValue>,
    mask_value: &super::board::CellValue,
) -> u8 {
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

fn has_winning_bits(bits: &u8) -> bool {
    return bits & 15 == 15
        || bits & 30 == 30
        || bits & 60 == 60
        || bits & 120 == 120
        || bits & 240 == 240;
}

pub fn complete_game_if_finished(winner: &Winner) {
    if *winner == Winner::NOBODY {
        return;
    }

    if *winner == Winner::HUMAN {
        println!("Human wins!");
    } else if *winner == Winner::AI {
        println!("Computer wins!");
    } else {
        println!("Draft, no further moves possible!")
    }
    process::exit(0);
}

fn cell_value_to_winner(cell_value: &super::board::CellValue) -> Winner {
    if *cell_value == super::board::CellValue::HUMAN {
        return Winner::HUMAN;
    } else {
        return Winner::AI;
    }
}

pub fn get_winner(board: &super::board::Board, col: &u8, row: &u8) -> Winner {
    let cell_value = board.get_value(col, row);

    if *cell_value == super::board::CellValue::EMPTY {
        panic!("Last entered value should not be EMPTY.");
    }

    let column_bits = value_vector_to_bits(&board.get_column_as_value_vector(col), cell_value);
    if has_winning_bits(&column_bits) {
        return cell_value_to_winner(cell_value);
    }

    let row_bits = value_vector_to_bits(&board.get_row_as_value_vector(row), cell_value);
    if has_winning_bits(&row_bits) {
        return cell_value_to_winner(cell_value);
    }

    let diagonal_inc_bits = value_vector_to_bits(
        &board.get_rising_diagonal_as_value_vector(col, row),
        cell_value,
    );
    if has_winning_bits(&diagonal_inc_bits) {
        return cell_value_to_winner(cell_value);
    }

    let diagonal_decr_bits = value_vector_to_bits(
        &board.get_falling_diagonal_as_value_vector(col, row),
        cell_value,
    );
    if has_winning_bits(&diagonal_decr_bits) {
        return cell_value_to_winner(cell_value);
    }

    // check if there are any empty fields
    let top_row_idx = super::ROWS - 1;
    let top_row = board.get_row_as_value_vector(&top_row_idx);
    for cell_value in top_row {
        if *cell_value == super::board::CellValue::EMPTY {
            return Winner::NOBODY;
        }
    }

    return Winner::DRAFT;
}
