mod constants;
mod utils;
mod game_result;
mod game_move;
mod convert;
mod engine_base;
mod engine_impl;

use wasm_bindgen::prelude::*;
use serde::{Serialize};
use game_result::{black_wins, white_wins};
use game_move::{get_next_board, MoveResult};
use convert::{to_js_board};

/// コマを置いて次の局面を得る
#[wasm_bindgen]
pub fn put_piece(board: u64, is_black: bool, col_index: i32) -> JsValue {
    let optional_next_board = get_next_board(board, is_black, col_index);
    let next_board = optional_next_board.expect("optional_next_board is None");

    let game_result = if is_black && black_wins(next_board, col_index) { -1 }
        else if !is_black && white_wins(next_board, col_index) { 1 }
        else { 0 };
    
    let move_result = MoveResult {
        board: to_js_board(next_board),
        game_result: game_result
    };

    let serializer = serde_wasm_bindgen::Serializer::new()
        .serialize_large_number_types_as_bigints(true);
    return move_result.serialize(&serializer).unwrap();
}
