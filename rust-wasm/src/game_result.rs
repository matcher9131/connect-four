use crate::constants::{BOARD_SIZE, NUM_CONNECTION};
use crate::utils::{bit_parallel_fill_right};

const NUM_CANDIDATES: i32 = BOARD_SIZE - NUM_CONNECTION + 1;

const VERTICAL_MASK: u64 = 0b00001111;
const HORIZONTAL_MASK: u64 = 0b00000001_00000001_00000001_00000001;
const DIAGONAL_UR_MASK: u64 = 0b00001000_00000100_00000010_00000001;
const DIAGONAL_LR_MASK: u64 = 0b00000000_00100000_01000000_10000001;

fn wins(sided_board: u64) -> bool {
    // iが列番号、jが行番号
    for i in 0..BOARD_SIZE {
        for j in 0..NUM_CANDIDATES {
            let mask = VERTICAL_MASK << (8 * i + j);
            if (sided_board & mask) == mask {
                return true;
            }
        }
    }

    for i in 0..NUM_CANDIDATES {
        for j in 0..BOARD_SIZE {
            let mask = HORIZONTAL_MASK << (8 * i + j);
            if (sided_board & mask) == mask {
                return true;
            }
        }
    }

    for i in 0..NUM_CANDIDATES {
        for j in 0..NUM_CANDIDATES {
            let mask = DIAGONAL_UR_MASK << (8 * i + j);
            if (sided_board & mask) == mask {
                return true;
            }
        }
    }

    for i in 0..NUM_CANDIDATES {
        for j in NUM_CANDIDATES..BOARD_SIZE {
            let mask = DIAGONAL_LR_MASK << (8 * i + j);
            if (sided_board & mask) == mask {
                return true;
            }
        }
    }

    return false;
}

pub fn white_wins(board: u64) -> bool {
    let mask = bit_parallel_fill_right(board);
    let sided_board = board ^ mask;
    return wins(sided_board);
}

pub fn black_wins(board: u64) -> bool {
    let mut mask = bit_parallel_fill_right(board);
    mask ^= (mask & 0xFEFE_FEFE_FEFE_FEFE) >> 1;
    let sided_board = board ^ mask;
    return wins(sided_board);
}
