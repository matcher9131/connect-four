pub fn bit_parallel_fill_right(x: u64) -> u64 {
    let mut y = x;
    y |= (y & 0xFEFE_FEFE_FEFE_FEFE) >> 1;
    y |= (y & 0xFCFC_FCFC_FCFC_FCFC) >> 2;
    y |= (y & 0xF0F0_F0F0_F0F0_F0F0) >> 4;
    return y;
}