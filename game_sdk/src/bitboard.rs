pub const SHIFT_RIGHT_MASK: u64 = 9259542123273814144;
pub const SHIFT_LEFT_MASK: u64 = SHIFT_RIGHT_MASK >> 7;
pub const FINISH_LINES: [u64; 2] = [SHIFT_RIGHT_MASK, SHIFT_LEFT_MASK];

pub fn format_bitboard(bitboard: u64) -> String {
    let mut string = " ----------------\n".to_string();
    for y in 0..8 {
        string.push('|');
        for x in 0..8 {
            let bit = 1 << (x + y * 8);
            if bitboard & bit != 0 {
                string.push_str(" 1");
            } else {
                string.push_str("  ");
            }
        }
        string.push_str("|\n")
    }
    string.push_str(" ----------------");
    string
}
