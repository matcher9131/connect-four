use crate::constants::{BOARD_SIZE, Piece};
use crate::game_result::{get_white_sided_board, get_black_sided_board};

pub fn to_wasm_board(val: &[u8]) -> u64 {
    assert!(val.len() >= (BOARD_SIZE * BOARD_SIZE) as usize);

    let mut result: u64 = 0;
    for i in 0..BOARD_SIZE {
        let mut top = -1;
        for j in 0..BOARD_SIZE {
            if val[(BOARD_SIZE * i + j) as usize] == Piece::Black as u8 {
                result |= 1u64 << (8 * i + j);
            }
            if val[(BOARD_SIZE * i + j) as usize] != Piece::Empty as u8 {
                top = j;
            }
        }
        result |= 1u64 << (8 * i + top + 1) as u32;
    }
    return result;
}

pub fn to_js_board(val: u64) -> Vec<Piece> {
    let mut result = vec![Piece::Empty; (BOARD_SIZE * BOARD_SIZE) as usize];
    let white_sided_board = get_white_sided_board(val);
    let black_sided_board = get_black_sided_board(val);
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            let mask = 1u64 << (8 * i + j);
            if (white_sided_board & mask) == mask {
                result[(BOARD_SIZE * i + j) as usize] = Piece::White;
            } else if (black_sided_board & mask) == mask {
                result[(BOARD_SIZE * i + j) as usize] = Piece::Black;
            }
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // ビットボードの定義:
    //   列i・行jのビット位置: 8*i + j
    //   コマなし/白コマ = 0, 黒コマ = 1
    //   センチネルビット: 列が空なら bit(8*i)、非空なら bit(8*i + top + 1)

    // 全列が空の盤面: BOARD_SIZE = 7列、各列のbit(8*i)にセンチネル
    const EMPTY_BOARD: u64 =
        (1u64 << 0)  | (1u64 << 8)  | (1u64 << 16) | (1u64 << 24) |
        (1u64 << 32) | (1u64 << 40) | (1u64 << 48);

    const BLACK_PIECE: u8 = Piece::Black as u8;
    const WHITE_PIECE: u8 = Piece::White as u8;

    // (col, row, piece) の配列からval配列(JS側の盤面表現・u8)を生成する
    fn make_val(pieces: &[(usize, usize, u8)]) -> Vec<u8> {
        let mut val = vec![0u8; (BOARD_SIZE * BOARD_SIZE) as usize];
        for &(col, row, piece) in pieces {
            val[BOARD_SIZE as usize * col + row] = piece;
        }
        val
    }

    // (col, row, piece) の配列からPiece配列(to_js_board の返り値型)を生成する
    fn make_piece_val(pieces: &[(usize, usize, u8)]) -> Vec<Piece> {
        let mut val = vec![Piece::Empty; (BOARD_SIZE * BOARD_SIZE) as usize];
        for &(col, row, piece) in pieces {
            val[BOARD_SIZE as usize * col + row] = match piece {
                p if p == WHITE_PIECE => Piece::White,
                p if p == BLACK_PIECE => Piece::Black,
                _ => Piece::Empty,
            };
        }
        val
    }

    // --- to_wasm_board ---

    #[rstest]
    // 空盤面: 全列のbit(8*i)にセンチネルのみ
    #[case(make_val(&[]), EMPTY_BOARD)]
    // 列0・行0に黒コマ: bit 0 = 1(黒), センチネルbit 1 → byte = 0x03
    #[case(make_val(&[(0, 0, BLACK_PIECE)]), 0x0001_0101_0101_0103)]
    // 列0・行0に白コマ: bit 0 = 0(白), センチネルbit 1 → byte = 0x02
    #[case(make_val(&[(0, 0, WHITE_PIECE)]), 0x0001_0101_0101_0102)]
    // 列0・行0黒/行1白: bit 0 = 1, bit 1 = 0, センチネルbit 2 → byte = 0x05
    #[case(make_val(&[(0, 0, BLACK_PIECE), (0, 1, WHITE_PIECE)]), 0x0001_0101_0101_0105)]
    // 列0・行0白/行1黒: bit 0 = 0, bit 1 = 1, センチネルbit 2 → byte = 0x06
    #[case(make_val(&[(0, 0, WHITE_PIECE), (0, 1, BLACK_PIECE)]), 0x0001_0101_0101_0106)]
    // 列6・行0に黒コマ: bit 48 = 1, センチネルbit 49 → 上位バイト = 0x03
    #[case(make_val(&[(6, 0, BLACK_PIECE)]), 0x0003_0101_0101_0101)]
    // 列6・行6に黒コマ: bit 54 = 1, センチネルbit 55 → 上位バイト = 0xC0
    #[case(make_val(&[(6, 6, BLACK_PIECE)]), 0x00C0_0101_0101_0101)]
    // 列0が全て黒で満杯: bits 0-6 = 1, センチネルbit 7 → byte = 0xFF
    #[case(
        make_val(&[(0,0,BLACK_PIECE),(0,1,BLACK_PIECE),(0,2,BLACK_PIECE),
                   (0,3,BLACK_PIECE),(0,4,BLACK_PIECE),(0,5,BLACK_PIECE),(0,6,BLACK_PIECE)]),
        0x0001_0101_0101_01FF
    )]
    fn test_to_wasm_board(#[case] val: Vec<u8>, #[case] expected: u64) {
        assert_eq!(to_wasm_board(&val), expected);
    }

    #[test]
    #[should_panic]
    fn test_to_wasm_board_too_short() {
        let val = vec![0u8; (BOARD_SIZE * BOARD_SIZE) as usize - 1];
        to_wasm_board(&val);
    }

    // --- to_js_board ---

    #[rstest]
    // 空盤面
    #[case(EMPTY_BOARD, make_piece_val(&[]))]
    // 列0・行0に黒コマ
    #[case(0x0001_0101_0101_0103, make_piece_val(&[(0, 0, BLACK_PIECE)]))]
    // 列0・行0に白コマ
    #[case(0x0001_0101_0101_0102, make_piece_val(&[(0, 0, WHITE_PIECE)]))]
    // 列0・行0黒/行1白
    #[case(0x0001_0101_0101_0105, make_piece_val(&[(0, 0, BLACK_PIECE), (0, 1, WHITE_PIECE)]))]
    // 列6・行0に黒コマ
    #[case(0x0003_0101_0101_0101, make_piece_val(&[(6, 0, BLACK_PIECE)]))]
    fn test_to_js_board(#[case] board: u64, #[case] expected: Vec<Piece>) {
        assert_eq!(to_js_board(board), expected);
    }

    // --- ラウンドトリップ: to_js_board(to_wasm_board(val)) == val ---
    // 注意: エンコーディングは白コマと空マスを区別しないため、
    //       各列は row 0 から連続して積まれた有効な盤面のみ成立する

    #[rstest]
    // 空盤面
    #[case(make_val(&[]))]
    // 単一コマ
    #[case(make_val(&[(0, 0, BLACK_PIECE)]))]
    #[case(make_val(&[(0, 0, WHITE_PIECE)]))]
    // 同一列に黒・白を交互に積む (row 0 から連続)
    #[case(make_val(&[(0, 0, BLACK_PIECE), (0, 1, WHITE_PIECE), (0, 2, BLACK_PIECE)]))]
    // 複数列にまたがる (各列は row 0 から連続)
    #[case(make_val(&[(2, 0, WHITE_PIECE), (2, 1, BLACK_PIECE), (5, 0, WHITE_PIECE)]))]
    // 端の列同士 (各列はrow 0のみ)
    #[case(make_val(&[(0, 0, WHITE_PIECE), (6, 0, BLACK_PIECE)]))]
    fn test_roundtrip(#[case] val: Vec<u8>) {
        let result: Vec<u8> = to_js_board(to_wasm_board(&val)).iter().map(|p| *p as u8).collect();
        assert_eq!(result, val);
    }
}