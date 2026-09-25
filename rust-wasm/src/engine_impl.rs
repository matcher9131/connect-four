use rand::SeedableRng;
use rand::rngs::SmallRng;

use crate::clock::now_ms;
use crate::constants::{BOARD_SIZE, CLOCK_CHCECK_INTERVAL, EXPLORATION_CONSTANT, NUM_ITERATION};
use crate::engine_base::{GameState, mcts_search};
use crate::game_move::{get_next_board, get_next_row_index, board_is_full};
use crate::game_result::{black_wins, white_wins, wins_if_put};

#[derive(Clone)]
struct ConnectFour {
    board: u64,
    is_black: bool,
}

impl GameState for ConnectFour {
    type Action = i32;

    fn get_legal_actions(&self) -> Vec<Self::Action> {
        (0..BOARD_SIZE).filter(|&col_index| get_next_row_index(self.board, col_index) != -1).collect()
    }

    fn apply(&self, action: Self::Action) -> Self {
        let next_board = get_next_board(self.board, self.is_black, action);
        ConnectFour { board: next_board.expect("Chosen invalid col_index"), is_black: !self.is_black }
    }

    fn get_terminal_value(&self) -> Option<f64> {
        // 勝ち
        for col_index in 0..BOARD_SIZE {
            if wins_if_put(self.board, self.is_black, col_index) {
                return Some(1.0);
            }
        }
        // 引き分け
        if board_is_full(self.board) {
            return Some(0.0);
        }
        // 未決着
        None
    }
}

pub fn get_next_move(board: u64, cpu_is_black: bool, time_limit_ms: f64, seed: u64) -> i32 {
    // 即座に勝てる手があるなら探索せずにそれを返す
    for col_index in 0..BOARD_SIZE {
        if wins_if_put(board, cpu_is_black, col_index) {
            return col_index;
        }
    }

    let deadline = now_ms() + time_limit_ms;
    let mut rng = SmallRng::seed_from_u64(seed);

    // モンテカルロ木探索で探索する
    mcts_search(
        ConnectFour { board, is_black: cpu_is_black }, 
        EXPLORATION_CONSTANT, 
        &mut rng,
        |i| i >= NUM_ITERATION || (i % CLOCK_CHCECK_INTERVAL == 0 && now_ms() >= deadline),
    ).unwrap_or(-1)
}
