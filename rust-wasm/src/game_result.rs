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

/// 現在の局面から特定の位置に白のコマを置くと白が勝利条件を満たすかどうかを返す
/// 
/// # Arguments
/// - `board` - 局面
/// - `col_index` - コマを置く列
/// - `row_index` - コマを置く行
pub fn white_wins(board: u64, col_index: i32, row_index: i32) -> bool {
    let mask = bit_parallel_fill_right(board);
    let sided_board = board ^ mask;
    return wins(sided_board, col_index, row_index);
}

/// 現在の局面から特定の位置に黒のコマを置くと黒が勝利条件を満たすかどうかを返す
/// 
/// # Arguments
/// - `board` - 局面
/// - `col_index` - コマを置く列
/// - `row_index` - コマを置く行
pub fn black_wins(board: u64, col_index: i32, row_index: i32) -> bool {
    let mut mask = bit_parallel_fill_right(board);
    mask ^= (mask & 0xFEFE_FEFE_FEFE_FEFE) >> 1;
    let sided_board = board ^ mask;
    return wins(sided_board, col_index, row_index);
}


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // 列i・行jのビット位置: 8*i + j
    const VERT_WIN: u64 = 0x0F;         // col 0, rows 0-3
    const HORIZ_WIN: u64 = 0x0101_0101; // row 0, cols 0-3
    // (col,row): (0,0)→(1,1)→(2,2)→(3,3): bits 0,9,18,27
    const DIAG_UR_WIN: u64 = (1u64 << 0) | (1u64 << 9) | (1u64 << 18) | (1u64 << 27);
    // (col,row): (0,3)→(1,2)→(2,1)→(3,0): bits 3,10,17,24
    const DIAG_LR_WIN: u64 = (1u64 << 3) | (1u64 << 10) | (1u64 << 17) | (1u64 << 24);

    #[rstest]
    // 縦4連: col 0, rows 0-3
    #[case(VERT_WIN, 0, 0, true)]
    #[case(VERT_WIN, 0, 3, true)]
    // 縦4連: col 0, rows 3-6 (0x78 = bits 3-6)
    #[case(0x78u64, 0, 3, true)]
    #[case(0x78u64, 0, 6, true)]
    // 横4連: row 0, cols 0-3
    #[case(HORIZ_WIN, 0, 0, true)]
    #[case(HORIZ_WIN, 3, 0, true)]
    // 横4連: row 0, cols 3-6
    #[case(HORIZ_WIN << 24, 3, 0, true)]
    #[case(HORIZ_WIN << 24, 6, 0, true)]
    // 右上がり斜め4連: (0,0)-(3,3)
    #[case(DIAG_UR_WIN, 0, 0, true)]
    #[case(DIAG_UR_WIN, 3, 3, true)]
    // 右上がり斜め4連: (0,1)-(3,4)
    #[case(DIAG_UR_WIN << 1, 2, 3, true)]
    #[case(DIAG_UR_WIN << 1, 3, 4, true)]
    // 右下がり斜め4連: (0,3)-(3,0)
    #[case(DIAG_LR_WIN, 0, 3, true)]
    #[case(DIAG_LR_WIN, 3, 0, true)]
    // 非勝利: 空盤面
    #[case(0u64, 0, 0, false)]
    // 非勝利: col 0 に3連のみ
    #[case(0x07u64, 0, 2, false)]
    // 非勝利: row 0 に横3連のみ (cols 0-2: bits 0,8,16)
    #[case((1u64 << 0) | (1u64 << 8) | (1u64 << 16), 0, 0, false)]
    fn test_wins(#[case] board: u64, #[case] col: i32, #[case] row: i32, #[case] expected: bool) {
        assert_eq!(wins(board, col, row), expected);
    }
}