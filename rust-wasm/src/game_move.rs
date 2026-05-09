use wasm_bindgen::prelude::*;
use serde::{Serialize};

#[wasm_bindgen]
#[derive(Serialize)]
pub struct MoveResult {
    /// 次の局面
    pub board: u64,

    /// 先手勝ちなら`1`、後手勝ちなら`-1`、未決着なら`0`
    pub game_result: i32
}

/// 次に指定した列にコマを置く場合、それが何行目になるかを返す
/// 置けないときは`-1`を返す
pub fn get_next_row_index(board: u64, col_index: i32) -> i32 {
    let col = (board >> (8 * col_index)) & 0xFFu64;
    let row_index = col.ilog2() as i32;
    return if row_index < 7 { row_index } else { -1 };
}

/// 次の局面を作って返す
pub fn get_next_board(board: u64, is_black: bool, col_index: i32) -> u64 {
    let row_index = get_next_row_index(board, col_index);
    let piece_bits = if is_black { 0 } else { 1u64 << (8 * col_index + row_index) };
    return (board ^ piece_bits) | (1u64 << (8 * col_index + row_index + 1));
}