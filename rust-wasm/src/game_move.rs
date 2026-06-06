use serde::{Serialize};

use crate::constants::Piece;

#[derive(Serialize)]
pub struct MoveResult {
    /// 次の局面
    pub board: Vec<Piece>,

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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // ボードのビット配置: 列i・行jのビット位置 = 8*i + j
    // 各列の最下位バイトにセンチネルビット(bit 0)が置かれた空盤面
    const EMPTY_COL0: u64 = 0x01; // 列0のみセンチネル(bit 0)

    #[rstest]
    // 空の列0: センチネルがbit 0 → 次の行は0
    #[case(EMPTY_COL0, 0, 0)]
    // 列0に1コマ後(センチネルがbit 1): 次の行は1
    #[case(0x02u64, 0, 1)]
    // 列0に1コマ後(黒コマ+センチネル): 次の行は1
    #[case(0x03u64, 0, 1)]
    // 列0にセンチネルがbit 6 → 次の行は6
    #[case(0x40u64, 0, 6)]
    // 列0が満杯(センチネルがbit 7) → -1
    #[case(0x80u64, 0, -1)]
    #[case(0xFFu64, 0, -1)]
    // 別の列: 列1のセンチネルがbit 8 → 次の行は0
    #[case(0x0100u64, 1, 0)]
    // 列1にセンチネルがbit 9 → 次の行は1
    #[case(0x0200u64, 1, 1)]
    // 列1が満杯(センチネルがbit 15) → -1
    #[case(0x8000u64, 1, -1)]
    // 複数列が混在しても対象列のみ参照
    #[case(0x8001u64, 0, 0)] // 列0: bit 0 → 0, 列1: bit 15 (満杯は無関係)
    fn test_get_next_row_index(
        #[case] board: u64,
        #[case] col_index: i32,
        #[case] expected: i32,
    ) {
        assert_eq!(get_next_row_index(board, col_index), expected);
    }

    #[rstest]
    // 空の列0(センチネルbit 0)に黒コマ → センチネルbit 1 + 元のbit 0(黒=1)はそのまま
    // bit 0 = 1(黒), bit 1 = 1(センチネル) → 0x03
    #[case(EMPTY_COL0, true, 0, 0x03u64)]
    // 空の列0に白コマ → センチネルがbit 1へ移動、bit 0 = 0(白)
    // bit 0 XOR 1 = 0(白), bit 1 = 1(センチネル) → 0x02
    #[case(EMPTY_COL0, false, 0, 0x02u64)]
    // 列0(黒コマ行0+センチネルbit 1)に黒コマ → センチネルbit 2追加
    // 0x03 | 0x04 = 0x07
    #[case(0x03u64, true, 0, 0x07u64)]
    // 列0(黒コマ行0+センチネルbit 1)に白コマ
    // bit 1 XOR 1 = 0(白), bit 2 = 1(センチネル), bit 0 = 1(黒) → 0x05
    #[case(0x03u64, false, 0, 0x05u64)]
    // 列0(白コマ行0+センチネルbit 1)に黒コマ
    // 0x02 | 0x04 = 0x06
    #[case(0x02u64, true, 0, 0x06u64)]
    // 列0(白コマ行0+センチネルbit 1)に白コマ
    // bit 1 XOR 1 = 0(白), bit 2 = 1(センチネル) → 0x04
    #[case(0x02u64, false, 0, 0x04u64)]
    // 別の列: 列1(センチネルbit 8)に黒コマ
    // bit 8 = 1(黒), bit 9 = 1(センチネル) → 0x0300
    #[case(0x0100u64, true, 1, 0x0300u64)]
    // 列1(センチネルbit 8)に白コマ → 0x0200
    #[case(0x0100u64, false, 1, 0x0200u64)]
    // 他の列は変化しない: 列0と列1が混在
    #[case(0x0101u64, true, 0, 0x0103u64)]  // 列1は変化なし
    #[case(0x0101u64, true, 1, 0x0301u64)]  // 列0は変化なし
    fn test_get_next_board(
        #[case] board: u64,
        #[case] is_black: bool,
        #[case] col_index: i32,
        #[case] expected: u64,
    ) {
        assert_eq!(get_next_board(board, is_black, col_index), expected);
    }
}