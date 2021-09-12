use super::action::UndoInfo;
use super::bitboard::bitboard_to_string;
use super::piece;
use rand::{rngs::SmallRng, RngCore, SeedableRng};
use std::fmt::{Display, Formatter, Result};

pub const RED: usize = 0;
pub const BLUE: usize = 1;
pub const COLORS: [usize; 2] = [RED, BLUE];

#[derive(Clone)]
pub struct GameState {
    pub ply: u8,
    pub board: [[u64; 4]; 2],
    pub occupied: [u64; 2],
    pub stacked: u64,
    pub ambers: [u8; 2],
    pub undo: [UndoInfo; 64],
}

impl GameState {
    pub fn empty() -> Self {
        Self {
            ply: 0,
            board: [[0u64; 4]; 2],
            occupied: [0u64; 2],
            stacked: 0u64,
            ambers: [0u8; 2],
            undo: [UndoInfo::default(); 64],
        }
    }

    pub fn random() -> Self {
        let mut rng = SmallRng::from_entropy();
        let mut state = GameState::empty();
        let mut pieces_left = [2, 2, 2, 2];
        for y in 0..8 {
            loop {
                let random_piece_type = rng.next_u64() as usize % 4;
                if pieces_left[random_piece_type] == 0 {
                    continue;
                }
                pieces_left[random_piece_type] -= 1;
                let red_position = 1 << (y * 8);
                state.board[RED][random_piece_type] |= red_position;
                state.occupied[RED] |= red_position;
                let blue_position = 1 << ((7 - y) * 8 + 7);
                state.board[BLUE][random_piece_type] |= blue_position;
                state.occupied[BLUE] |= blue_position;
                break;
            }
        }
        state
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
            ambers: [0u8; 2],
            undo: [UndoInfo::default(); 64],
        };
        for color in COLORS.iter() {
            for piece in piece::PIECES.iter() {
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
            self.board[RED][piece::COCKLE as usize],
            self.board[RED][piece::GULL as usize],
            self.board[RED][piece::STARFISH as usize],
            self.board[RED][piece::SEAL as usize],
            self.board[BLUE][piece::COCKLE as usize],
            self.board[BLUE][piece::GULL as usize],
            self.board[BLUE][piece::STARFISH as usize],
            self.board[BLUE][piece::SEAL as usize],
            self.stacked,
            self.ambers[0] | self.ambers[1] << 4,
        )
    }

    pub fn check_integrity(&self) -> bool {
        let mut occupied: [u64; 2] = [0; 2];
        for color in COLORS {
            for piece_type in 0..4 {
                occupied[color] |= self.board[color][piece_type];
            }
            if occupied[color] != self.occupied[color] {
                println!(
                    "Something went wrong with the occupancy map for color {}",
                    color
                );
                println!(
                    "It should be:\n{}\nNot\n{}",
                    bitboard_to_string(occupied[color]),
                    bitboard_to_string(self.occupied[color])
                );
                return false;
            }
        }
        if occupied[0] & occupied[1] > 0 {
            println!("There is a field which is owned by both colors.");
            println!(
                "RED\n{}\nBLUE\n{}",
                bitboard_to_string(self.occupied[RED]),
                bitboard_to_string(self.occupied[BLUE])
            );
            return false;
        }
        if self.stacked & (occupied[0] | occupied[1]) != self.stacked {
            println!("A field that contains a stack must be occupied by a piece.");
            println!(
                "Stacked:\n{}\nOccupied:\n{}",
                bitboard_to_string(self.stacked),
                bitboard_to_string(occupied[0] | occupied[1])
            );
            return false;
        }
        true
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
            "║ {} Turn: {} {}:{}",
            match self.get_current_color() {
                0 => "🟥",
                _ => "🟦",
            },
            self.ply,
            self.ambers[0],
            self.ambers[1],
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
                for color in COLORS {
                    for piece in piece::PIECES {
                        if self.board[color][piece as usize] & bit != 0 {
                            let stacked = if self.stacked & bit > 0 { '+' } else { ' ' };
                            string.push_str(&format!(
                                " {}{} ",
                                piece::to_char(piece, color),
                                stacked
                            ));
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
