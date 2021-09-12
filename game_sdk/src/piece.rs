pub const COCKLE: u8 = 0;
pub const GULL: u8 = 1;
pub const STARFISH: u8 = 2;
pub const SEAL: u8 = 3;
pub const PIECES: [u8; 4] = [COCKLE, GULL, STARFISH, SEAL];

pub fn to_char(piece: u8, color: usize) -> char {
    let chars: [[char; 4]; 2] = [['C', 'G', 'F', 'S'], ['c', 'g', 'f', 's']];
    chars[color][piece as usize]
}
