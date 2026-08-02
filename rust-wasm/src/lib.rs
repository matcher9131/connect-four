mod constants;
mod utils;
mod game_result;
mod game_move;
mod convert;
mod engine_base;
mod engine_impl;
mod clock;

use wasm_bindgen::prelude::*;
use serde::{Serialize};

use crate::convert::{to_js_board, to_wasm_board};
use crate::game_result::{get_game_result};
use crate::game_move::{MoveResult, get_next_board};

/// コマを置いて次の局面を得る
#[wasm_bindgen]
pub fn put_piece(js_board: &[u8], is_black: bool, col_index: i32) -> JsValue {
    let board = to_wasm_board(js_board);
    let next_board = get_next_board(board, is_black, col_index).expect("optional_next_board is None");
    let move_result = MoveResult {
        board: to_js_board(next_board),
        game_result: get_game_result(next_board, is_black, col_index)
    };

    let serializer = serde_wasm_bindgen::Serializer::new()
        .serialize_large_number_types_as_bigints(true);
    move_result.serialize(&serializer).unwrap()
}

/// CPUに思考させて指し手を反映した局面を返す
#[wasm_bindgen]
pub fn get_next_board_by_cpu(js_board: &[u8], cpu_is_black: bool, time_limit_ms: f64, seed: u32) -> JsValue {
    let board = to_wasm_board(js_board);
    let col_index = engine_impl::get_next_move(board, cpu_is_black, time_limit_ms, seed as u64);
    let next_board = get_next_board(board, cpu_is_black, col_index).expect("board cannot be full");
    let move_result = MoveResult {
        board: to_js_board(next_board),
        game_result: get_game_result(next_board, cpu_is_black, col_index)
    };

    let serializer = serde_wasm_bindgen::Serializer::new()
        .serialize_large_number_types_as_bigints(true);
    move_result.serialize(&serializer).unwrap()
}
