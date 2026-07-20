use serde_repr::Serialize_repr;

pub const BOARD_SIZE: i32 = 7;
pub const NUM_CONNECTION: i32 = 4;

#[derive(Serialize_repr, Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub enum Piece {
    Empty = 0u8,
    White = 1u8,
    Black = 2u8,
}

pub const NUM_ITERATION: u32 = 100_000;
pub const EXPLORATION_CONSTANT: f64 = 1.4;
