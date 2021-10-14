use super::search::MATE_VALUE;
use game_sdk::bitboard::bitboard_to_string;
use game_sdk::gamerules::{self, *};
use game_sdk::gamestate::*;
use game_sdk::piece::*;

//pub const PIECE_VALUE_TABLE: [[[f32; 64]; 4]; 2] = [[[0.; 64]; 4]; 2];
/*let mut piece_value: f32 = 0.;
for piece in 0..4 {
    let mut pieces = state.board[color][piece];
    while pieces > 0 {
        let position = pieces.trailing_zeros();
        pieces ^= 1 << position;
        piece_value += PIECE_VALUE_TABLE[color][piece][position as usize];
    }
}*/

pub const DEFAULT_PARAMETERS: [f32; 4] = [20., 100., 1., 3.];

pub fn get_reachable_fields(board: &[u64; 4], color: usize) -> u64 {
    let mut reachable_fields: u64 = 0;
    let color = color << 6;
    let mut cockles = board[COCKLE as usize];
    while cockles > 0 {
        let position = cockles.trailing_zeros();
        cockles ^= 1 << position;
        reachable_fields |= COCKLE_PATTERN[position as usize | color];
    }
    let mut gulls = board[GULL as usize];
    while gulls > 0 {
        let position = gulls.trailing_zeros();
        gulls ^= 1 << position;
        reachable_fields |= GULL_PATTERN[position as usize];
    }
    let mut starfish = board[STARFISH as usize];
    while starfish > 0 {
        let position = starfish.trailing_zeros();
        starfish ^= 1 << position;
        reachable_fields |= STARFISH_PATTERN[position as usize | color];
    }
    let mut seals = board[SEAL as usize];
    while seals > 0 {
        let position = seals.trailing_zeros();
        seals ^= 1 << position;
        reachable_fields |= SEAL_PATTERN[position as usize];
    }
    reachable_fields
}
/*let mut value: f32 = 0.;
let other_color = color ^ 0b1;
let ambers = state.ambers[color] as f32;
value += ambers * parameters[0];

let pieces_that_can_be_captured = state.occupied[other_color] & colors_reachable_fields;
let stacked_pieces_that_can_be_captured = state.stacked & pieces_that_can_be_captured;
if stacked_pieces_that_can_be_captured > 0 && ambers >= 1.0 {
    if color == BLUE as usize || state.ambers[other_color] == 0 {
        return MATE_VALUE as f32;
    } else {
        let a = state.occupied[color] & other_colors_reachable_fields & state.stacked;
        if a == 0 {
            return MATE_VALUE as f32;
        }
    }
}
value*/

pub fn evaluate_color(
    state: &GameState,
    color: usize,
    is_colors_turn: bool,
    colors_reachable_fields: u64,
    other_colors_reachable_fields: u64,
    parameters: &[f32; 4],
) -> f32 {
    let mut value: f32 = 0.;
    let other_color = color ^ 1;

    let stacked_pieces = (state.stacked & state.occupied[color]).count_ones() as f32;
    value += stacked_pieces * parameters[0];

    let ambers = state.ambers[color] as f32;
    value += ambers * parameters[1];

    let covered = state.occupied[other_color] & colors_reachable_fields;
    let save = covered & !state.stacked;
    value += (save.count_ones() as f32) * parameters[2];

    if is_colors_turn {
        value += parameters[3];
    }
    value
}

pub fn static_evaluation(state: &GameState) -> i16 {
    let is_reds_turn = state.get_current_color() == RED as usize;
    let red_reachable_fields = get_reachable_fields(&state.board[RED as usize], RED as usize);
    let blue_reachable_fields = get_reachable_fields(&state.board[BLUE as usize], BLUE as usize);
    let red = evaluate_color(
        state,
        RED as usize,
        is_reds_turn,
        red_reachable_fields,
        blue_reachable_fields,
        &DEFAULT_PARAMETERS,
    ) as i16;
    let blue = evaluate_color(
        state,
        BLUE as usize,
        !is_reds_turn,
        blue_reachable_fields,
        red_reachable_fields,
        &DEFAULT_PARAMETERS,
    ) as i16;
    red - blue
}
