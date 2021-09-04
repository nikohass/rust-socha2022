pub const PIECES: [Piece; 4] = [Piece::Cockle, Piece::Gull, Piece::Starfish, Piece::Seal];

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Piece {
    Cockle = 0,
    Gull = 1,
    Starfish = 2,
    Seal = 3,
}

impl Piece {
    pub fn to_char(&self, color: usize) -> char {
        let chars: [[char; 4]; 2] = [['C', 'G', 'F', 'S'], ['c', 'g', 'f', 's']];
        chars[color][*self as usize]
    }
}
