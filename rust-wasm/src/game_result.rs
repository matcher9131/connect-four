use crate::constants::{BOARD_SIZE, NUM_CONNECTION};
use crate::utils::{bit_parallel_fill_right};

const UPPER_LIMIT: i32 = BOARD_SIZE - NUM_CONNECTION;

const VERTICAL_MASK: u64 = 0b00001111;
const HORIZONTAL_MASK: u64 = 0b00000001_00000001_00000001_00000001;
const DIAGONAL_UR_MASK: u64 = 0b00001000_00000100_00000010_00000001;
const DIAGONAL_LR_MASK: u64 = 0b00000000_00100000_01000000_10000001;

fn wins(sided_board: u64, col_index: i32, row_index: i32) -> bool {
    assert!(0 <= col_index && col_index < BOARD_SIZE);
    assert!(0 <= row_index && row_index < BOARD_SIZE);

    let down = (row_index - NUM_CONNECTION + 1).max(0);
    let up = row_index.min(UPPER_LIMIT);
    assert!(0 <= down && down <= up && up <= UPPER_LIMIT);
    for j in down..=up {
        let mask = VERTICAL_MASK << (8 * col_index + j);
        if (sided_board & mask) == mask {
            return true;
        }
    }

    let left = (col_index - NUM_CONNECTION + 1).max(0);
    let right = col_index.min(UPPER_LIMIT);
    assert!(0 <= left && left <= right && right <= UPPER_LIMIT);
    for i in left..=right {
        let mask = HORIZONTAL_MASK << (8 * i + row_index);
        if (sided_board & mask) == mask {
            return true;
        }
    }

    // 0 <= c+k <= UPPER_LIMIT と 0 <= r+k <= UPPER_LIMIT の共通範囲
    let ur_k_min = (-NUM_CONNECTION + 1).max(-col_index).max(-row_index);
    let ur_k_max = (NUM_CONNECTION - 1).min(UPPER_LIMIT - col_index).min(UPPER_LIMIT - row_index);
    for k in ur_k_min..=ur_k_max {
        debug_assert!(0 <= col_index + k && col_index + k <= UPPER_LIMIT);
        debug_assert!(0 <= row_index + k && row_index + k <= UPPER_LIMIT);
        let mask = DIAGONAL_UR_MASK << (8 * (col_index + k) + row_index + k);
        if (sided_board & mask) == mask {
            return true;
        }
    }

    // 0 <= c+k <= UPPER_LIMIT と UPPER_LIMIT <= r-k < BOARD_SIZE の共通範囲
    let lr_k_min = (-NUM_CONNECTION + 1).max(-col_index).max(row_index - BOARD_SIZE + 1);
    let lr_k_max = (NUM_CONNECTION - 1).min(UPPER_LIMIT - col_index).min(row_index - UPPER_LIMIT);
    for k in lr_k_min..=lr_k_max {
        debug_assert!(0 <= col_index + k && col_index + k < UPPER_LIMIT);
        debug_assert!(UPPER_LIMIT <= row_index - k && row_index - k < BOARD_SIZE);
        let mask = DIAGONAL_LR_MASK << (8 * (col_index + k) + (row_index - k));
        if (sided_board & mask) == mask {
            return true;
        }
    }

    return false;
}

pub fn white_wins(board: u64, col_index: i32, row_index: i32) -> bool {
    let mask = bit_parallel_fill_right(board);
    let sided_board = board ^ mask;
    return wins(sided_board, col_index, row_index);
}

pub fn black_wins(board: u64, col_index: i32, row_index: i32) -> bool {
    let mut mask = bit_parallel_fill_right(board);
    mask ^= (mask & 0xFEFE_FEFE_FEFE_FEFE) >> 1;
    let sided_board = board ^ mask;
    return wins(sided_board, col_index, row_index);
}
