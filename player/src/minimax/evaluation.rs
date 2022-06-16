use game_sdk::bitboard::*;
use game_sdk::gamerules::*;
use game_sdk::gamestate::*;
use game_sdk::piece::*;

pub const MATE_VALUE: i16 = 31_000;

struct ReachableFields {
    cockle: u64,
    cockle_stack: u64,
    gull: u64,
    gull_stack: u64,
    starfish: u64,
    starfish_stack: u64,
    seal: u64,
    seal_stack: u64,
    all: u64,
    all_stacked: u64,
}

impl Default for ReachableFields {
    fn default() -> Self {
        ReachableFields {
            cockle: 0,
            cockle_stack: 0,
            gull: 0,
            gull_stack: 0,
            starfish: 0,
            starfish_stack: 0,
            seal: 0,
            seal_stack: 0,
            all: 0,
            all_stacked: 0,
        }
    }
}

impl ReachableFields {
    pub fn for_color(color: usize, board: &[u64; 4], stacked: u64) -> Self {
        let color = (color as usize) << 6;
        let mut reachable_fields = ReachableFields::default();
        // Get reachable fileds for cockles
        let mut cockles = board[COCKLE as usize];
        while cockles > 0 {
            let position = cockles.trailing_zeros();
            let bit = 1 << position;
            cockles ^= bit;
            if bit & stacked != 0 {
                reachable_fields.cockle_stack |= bit;
            }
            reachable_fields.cockle |= COCKLE_PATTERN[position as usize | color];
        }
        // Get reachable fields for gulls
        let mut gulls = board[GULL as usize];
        while gulls > 0 {
            let position = gulls.trailing_zeros();
            let bit = 1 << position;
            gulls ^= bit;
            if bit & stacked != 0 {
                reachable_fields.gull_stack |= bit;
            }
            reachable_fields.gull |= GULL_PATTERN[position as usize];
        }
        // Get reachable fields for starfish
        let mut starfish = board[STARFISH as usize];
        while starfish > 0 {
            let position = starfish.trailing_zeros();
            let bit = 1 << position;
            starfish ^= bit;
            if bit & stacked != 0 {
                reachable_fields.starfish_stack |= bit;
            }
            reachable_fields.starfish |= STARFISH_PATTERN[position as usize | color];
        }
        // Get reachable fields for seals
        let mut seals = board[SEAL as usize];
        while seals > 0 {
            let position = seals.trailing_zeros();
            let bit = 1 << position;
            seals ^= bit;
            if bit & stacked != 0 {
                reachable_fields.seal_stack |= bit;
            }
            reachable_fields.seal |= SEAL_PATTERN[position as usize];
        }
        reachable_fields.all = reachable_fields.cockle
            | reachable_fields.gull
            | reachable_fields.starfish
            | reachable_fields.seal;
        reachable_fields.all_stacked = reachable_fields.cockle_stack
            | reachable_fields.gull_stack
            | reachable_fields.starfish_stack
            | reachable_fields.seal_stack;
        reachable_fields
    }
}

struct Captures {
    stack_captures: u64,
    captures_stack: u64,
    //stack_captures_stack: u64,
}

impl Captures {
    pub fn calculate(
        red_reachable_fields: &ReachableFields,
        red_occupied_fields: u64,
        blue_reachable_fields: &ReachableFields,
        blue_occupied_fields: u64,
        stacked: u64,
    ) -> (Self, Self) {
        (
            Self {
                stack_captures: red_reachable_fields.all & blue_occupied_fields & stacked,
                captures_stack: red_reachable_fields.all_stacked & blue_occupied_fields,
            },
            Self {
                stack_captures: blue_reachable_fields.all & red_occupied_fields & stacked,
                captures_stack: blue_reachable_fields.all_stacked & red_occupied_fields,
            },
        )
    }
}

/*fn save_capture(
    current_player_reachable_fields: &ReachableFields,
    current_player_occupied_fields: u64,
    other_player_reachable_fields: &ReachableFields,
    other_player_occupied_fields: u64,
    stacked: u64,
) -> bool {
    let stacked_pieces_that_i_can_capture =
        current_player_reachable_fields.all & other_player_occupied_fields & stacked;
    let pieces_that_i_can_capture_with_a_stacked_piece =
        current_player_reachable_fields.all_stacked & other_player_occupied_fields;
    stacked_pieces_that_i_can_capture | pieces_that_i_can_capture_with_a_stacked_piece > 0
}*/

pub fn static_evaluation(state: &GameState) -> i16 {
    let red_reachable_fields = ReachableFields::for_color(RED, &state.board[RED], state.stacked);
    let blue_reachable_fields = ReachableFields::for_color(BLUE, &state.board[BLUE], state.stacked);
    let is_reds_turn = state.ply % 2 == 0;
    let (red_captures, blue_captures) = Captures::calculate(
        &red_reachable_fields,
        state.occupied[RED],
        &blue_reachable_fields,
        state.occupied[BLUE],
        state.stacked,
    );
    // Check whether the current player has a winning move
    if is_reds_turn {
        if state.ambers[RED] == 1
            && ((red_captures.stack_captures > 0 || red_captures.captures_stack > 0)
                || (red_reachable_fields.gull
                    | red_reachable_fields.cockle
                    | red_reachable_fields.starfish)
                    & FINISH_LINES[RED]
                    > 0)
        {
            return MATE_VALUE;
        }
    } else if state.ambers[BLUE] == 1
        && ((blue_captures.stack_captures > 0 || blue_captures.captures_stack > 0)
            || (blue_reachable_fields.gull
                | blue_reachable_fields.cockle
                | blue_reachable_fields.starfish)
                & FINISH_LINES[BLUE]
                > 0)
    {
        return -MATE_VALUE;
    }

    if !is_reds_turn {
        if red_captures.stack_captures | red_captures.captures_stack > 1
            && state.ambers[RED] == 1
            && (state.ambers[BLUE] == 0
                || blue_captures.stack_captures | blue_captures.captures_stack == 0)
        {
            return MATE_VALUE;
        }
    } else if blue_captures.stack_captures | blue_captures.captures_stack > 1
        && state.ambers[BLUE] == 1
        && (state.ambers[RED] == 0
            || red_captures.stack_captures | red_captures.captures_stack == 0)
    {
        return -MATE_VALUE;
    }

    let capture_differnce = (red_captures.captures_stack | red_captures.stack_captures).count_ones()
        as f32
        - (blue_captures.captures_stack | blue_captures.stack_captures).count_ones() as f32
            * DEFAULT_PARAMETERS.capture_value;
    let stacked_pieces = ((state.stacked & state.occupied[RED]).count_ones() as f32
        - (state.stacked & state.occupied[BLUE]).count_ones() as f32)
        * DEFAULT_PARAMETERS.stacked_piece_value;
    let turn_advantage = DEFAULT_PARAMETERS.turn_advantage * if is_reds_turn { 1. } else { -1. };
    let amber_value =
        (state.ambers[RED] as f32 - state.ambers[BLUE] as f32) * DEFAULT_PARAMETERS.amber_value;
    let save = ((((state.occupied[BLUE] & red_reachable_fields.all) & !state.stacked).count_ones()
        as f32)
        - (((state.occupied[RED] & blue_reachable_fields.all) & !state.stacked).count_ones()
            as f32))
        * 1.0;
    (stacked_pieces + turn_advantage + amber_value + save + capture_differnce).round() as i16
}

struct EvaluationParameters {
    amber_value: f32,
    turn_advantage: f32,
    stacked_piece_value: f32,
    capture_value: f32,
}
const DEFAULT_PARAMETERS: EvaluationParameters = EvaluationParameters {
    amber_value: 100.0,
    turn_advantage: 3.0,
    stacked_piece_value: 20.0,
    capture_value: 10.0,
};

/*
pub fn evaluate_color(
    state: &GameState,
    color: usize,
    is_colors_turn: bool,
    colors_reachable_fields: u64,
    //other_colors_reachable_fields: u64,
) -> f32 {
    let parameters: [f32; 4] = [20., 100., 1., 3.];
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
}*/

/*
fn evaluate_color(
    state: &GameState,
    color: usize,
    is_my_turn: bool,
    my_reachable_fields: &ReachableFields,
    other_reachable_fields: &ReachableFields,
    my_captures: &Captures,
    other_captures: &Captures,
) -> f32 {
    let other_color = color ^ 1;
    let mut value: f32 = state.ambers[color] as f32 * DEFAULT_PARAMETERS.amber_value;
    //let covered = state.occupied[other_color] & my_reachable_fields.all;
    //let save = covered & !state.stacked;
    //value += (save.count_ones() as f32) * DEFAULT_PARAMETERS.save_value;
    value
}*/

/*
pub const DEFAULT_PARAMETERS: [f32; 4] = [20., 100., 1., 3.];

pub fn evaluate_color(
    state: &GameState,
    color: usize,
    is_colors_turn: bool,
    colors_reachable_fields: u64,
    //other_colors_reachable_fields: u64,
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
*/
/*
pub fn evaluate_state(state: &GameState) -> i16 {
    let red_reachable_fields =
        ReachableFields.for_color(RED, state.board[RED as usize], state.stacked);
    let blue_reachable_fields =
        ReachableFields.for_color(BLUE, state.board[BLUE as usize], state.stacked);
    evaluate_color(state, RED, red_reachable_fields, blue_reachable_fields)
        - evaluate_color(state, BLUE, blue_reachable_fields, red_reachable_fields)
}

pub fn evaluate_color(
    state: &GameState,
    color: Color,
    my_reachable_fields: ReachableFields,
    opponent_reachable_fields: ReachableFields,
) -> i16 {
    let is_my_turn = state.ply % 2 == color as u8;
    let my_color = color as usize;
    let mut my_ambers = state.ambers[my_color];
    let my_pieces = state.occupied[my_color];
    let opponent_pieces = state.occupied[opponent_color];
    if is_my_turn {
        let can_get_amber = (my_reachable_fields.all & opponent_pieces & stacked > 0)
            || (my_reachable_fields.all_stacked & opponent_pieces > 0);
        if can_get_amber {
            my_ambers += 1;
        }
        if my_ambers >= 2 {
            return 100000;
        }
    }*/
/*let opponent_color = (color as usize) ^ 1;
    let my_ambers = state.ambers[my_color];
    let other_ambers = state.ambers[opponent_color];

    let possible_captures = my_reachable_fields.all & opponent_pieces;
    let possible_stacked_captures = my_reachable_fields.all_stacked & opponent_pieces;
    if (possible_stacked_captures != 0 || possible_captures & stacked)
        && is_my_turn
        && my_ambers >= 1
    {
        return MATE_VALUE;
    }

    let value = my_ambers as f32 * DEFAULT_PARAMETERS.amber_value;
}*/

/*

//use super::search::MATE_VALUE;
//use game_sdk::bitboard::format_bitboard;
use game_sdk::gamerules::*;
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
    //other_colors_reachable_fields: u64,
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
        //blue_reachable_fields,
        &DEFAULT_PARAMETERS,
    ) as i16;
    let blue = evaluate_color(
        state,
        BLUE as usize,
        !is_reds_turn,
        blue_reachable_fields,
        //red_reachable_fields,
        &DEFAULT_PARAMETERS,
    ) as i16;
    red - blue
}
*/
