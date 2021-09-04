use super::piece::{Piece, PIECES};
use std::fmt::{Display, Formatter, Result};

pub const RED: usize = 0;
pub const BLUE: usize = 1;
pub const COLORS: [usize; 2] = [RED, BLUE];

pub struct GameState {
    pub ply: u8,
    pub board: [[u64; 4]; 2],
    pub occupied: [u64; 2],
    pub stacked: u64,
    pub ambers: [u8; 2],
}

impl GameState {
    pub fn empty() -> Self {
        Self {
            ply: 0,
            board: [[0u64; 4]; 2],
            occupied: [0u64; 2],
            stacked: 0u64,
            ambers: [0u8; 2],
        }
    }

    pub fn get_current_color(&self) -> usize {
        (self.ply & 0b1) as usize
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut entries: Vec<&str> = fen.split(' ').collect();
        let mut state = GameState {
            ply: entries.remove(0).parse::<u8>().unwrap(),
            board: [[0u64; 4]; 2],
            occupied: [0u64; 2],
            stacked: 0u64,
            ambers: [0u8; 2]
        };
        for color in COLORS.iter() {
            for piece in PIECES.iter() {
                let bitboard = entries.remove(0).parse::<u64>().unwrap();
                state.board[*color][*piece as usize] = bitboard;
                state.occupied[*color] |= bitboard;
            }
        }
        state.stacked = entries.remove(0).parse::<u64>().unwrap();
        let ambers = entries.remove(0).parse::<u8>().unwrap();
        state.ambers[0] = ambers & 0b1111;
        state.ambers[1] = ambers >> 4;
        state
    }

    pub fn to_fen(&self) -> String {
        format!(
            "{} {} {} {} {} {} {} {} {} {} {}",
            self.ply,
            self.board[RED][Piece::Cockle as usize],
            self.board[RED][Piece::Gull as usize],
            self.board[RED][Piece::Starfish as usize],
            self.board[RED][Piece::Seal as usize],
            self.board[BLUE][Piece::Cockle as usize],
            self.board[BLUE][Piece::Gull as usize],
            self.board[BLUE][Piece::Starfish as usize],
            self.board[BLUE][Piece::Seal as usize],
            self.stacked,
            self.ambers[0] | self.ambers[1] << 4,
        )
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut string = String::new();
        string.push('╔');
        for _ in 0..31 {
            string.push('═');
        }
        string.push_str("╗\n");
        let info = &format!(
            "║ {} Turn: {}",
            match self.get_current_color() {
                0 => "🟥",
                _ => "🟦",
            },
            self.ply,
        );
        string.push_str(info);
        for _ in info.len()..36 {
            string.push(' ');
        }
        string.push_str("║\n╠");
        for _ in 0..31 {
            string.push('═');
        }
        string.push_str("╣\n");
        for y in 0..8 {
            string.push('║');
            for x in 0..8 {
                let bit = 1 << (x + y * 8);
                let mut is_empty = true;
                for color in COLORS.iter() {
                    for piece in PIECES.iter() {
                        if self.board[*color][*piece as usize] & bit != 0 {
                            string.push_str(&format!(" {}  ", piece.to_char(*color)));
                            is_empty = false;
                            break;
                        }
                    }
                }
                if is_empty {
                    string.push_str(" .  ")
                }
            }
            string.pop();
            string.push_str("║\n║");
            for _ in 0..31 {
                string.push(' ');
            }
            string.push_str("║\n");
        }
        string.push('╚');
        for _ in 0..31 {
            string.push('═');
        }
        string.push('╝');
        write!(f, "{}", string)
    }
}
