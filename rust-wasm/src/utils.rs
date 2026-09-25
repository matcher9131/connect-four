/// 8ビットごとに区切り、それぞれの最上位ビット以下を全て1で埋めたものを返す
pub fn bit_parallel_fill_right(x: u64) -> u64 {
    let mut y = x;
    y |= (y & 0xFEFE_FEFE_FEFE_FEFE) >> 1;
    y |= (y & 0xFCFC_FCFC_FCFC_FCFC) >> 2;
    y |= (y & 0xF0F0_F0F0_F0F0_F0F0) >> 4;
    return y;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    // 全ビット0 → 変化なし
    #[case(0x00u64, 0x00u64)]
    // 全ビット1 → 変化なし
    #[case(0xFFu64, 0xFFu64)]
    // bit 0のみ → 変化なし (埋めるビットなし)
    #[case(0x01u64, 0x01u64)]
    // bit 1 → bits 1-0 を埋める
    #[case(0x02u64, 0x03u64)]
    // bit 3 → bits 3-0 を埋める
    #[case(0x08u64, 0x0Fu64)]
    // bit 4 → bits 4-0 を埋める
    #[case(0x10u64, 0x1Fu64)]
    // bit 5 → bits 5-0 を埋める
    #[case(0x20u64, 0x3Fu64)]
    // bit 6 → bits 6-0 を埋める
    #[case(0x40u64, 0x7Fu64)]
    // bit 7 → 全ビットを埋める
    #[case(0x80u64, 0xFFu64)]
    // 最上位ビットのみ考慮 (0b10100000 → bit 7 が最上位なので全て埋まる)
    #[case(0xA0u64, 0xFFu64)]
    // 複数バイト: 各バイト独立に処理
    #[case(0x0102u64, 0x0103u64)]
    #[case(0x8040u64, 0xFF7Fu64)]
    // 全バイトが 0x80 → 全バイトが 0xFF
    #[case(0x8080_8080_8080_8080u64, 0xFFFF_FFFF_FFFF_FFFFu64)]
    // 全バイトが 0x01 → 変化なし
    #[case(0x0101_0101_0101_0101u64, 0x0101_0101_0101_0101u64)]
    fn test_bit_parallel_fill_right(#[case] input: u64, #[case] expected: u64) {
        assert_eq!(bit_parallel_fill_right(input), expected);
    }
}