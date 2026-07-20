use crate::constants::{BOARD_SIZE, EXPLORATION_CONSTANT, NUM_ITERATION};
use crate::engine_base::{GameState, mcts_search};
use crate::game_move::{get_next_board, get_next_row_index};
use crate::game_result::{black_wins, white_wins};

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
        let f = if self.is_black { black_wins } else { white_wins };
        for col_index in 0..BOARD_SIZE {
            if f(self.board, col_index) {
                return Some(1.0);
            }
        }
        None
    }
}

pub fn get_next_move(board: u64, cpu_is_black: bool) -> i32 {
    // 即座に勝てる手があるなら探索せずにそれを返す
    let f = if cpu_is_black { black_wins } else { white_wins };
    for col_index in 0..BOARD_SIZE {
        if f(board, col_index) {
            return col_index;
        }
    }

    // モンテカルロ木探索で探索する
    let mut rng = rand::rng();
    mcts_search(
        ConnectFour { board, is_black: cpu_is_black }, 
        NUM_ITERATION, 
        EXPLORATION_CONSTANT, 
        &mut rng
    )
}
