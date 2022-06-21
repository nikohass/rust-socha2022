use game_sdk::bitboard::*;
use game_sdk::gamerules::*;
use game_sdk::gamestate::*;
use game_sdk::piece::*;

pub const MATE_VALUE: i16 = 31_000;

struct EvaluationParameters {
    amber_value: f32,
    turn_advantage: f32,
    stacked_piece_value: f32,
    capture_value: f32,
    reachable_fields_value: [f32; 4],
}
// TODO: Tune parameters
const DEFAULT_PARAMETERS: EvaluationParameters = EvaluationParameters {
    amber_value: 100.0,
    turn_advantage: 3.0,
    stacked_piece_value: 20.0,
    capture_value: 20.0,
    reachable_fields_value: [1.0, 1.0, 1.0, 1.0],
};

#[derive(Default)]
struct ReachableFields {
    //pieces: [u64; 4],
    //stacks: [u64; 4],
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
}

impl Captures {
    #[inline(always)]
    pub fn for_color(
        reachable_fields: &ReachableFields,
        opponent_occupied_fields: u64,
        stacked: u64,
    ) -> Self {
        Self {
            stack_captures: reachable_fields.all & opponent_occupied_fields & stacked,
            captures_stack: reachable_fields.all_stacked & opponent_occupied_fields,
        }
    }
}

pub fn static_evaluation(state: &GameState) -> i16 {
    let red_reachable_fields = ReachableFields::for_color(RED, &state.board[RED], state.stacked);
    let blue_reachable_fields = ReachableFields::for_color(BLUE, &state.board[BLUE], state.stacked);
    let is_reds_turn = state.ply % 2 == 0;
    let red_captures =
        Captures::for_color(&red_reachable_fields, state.occupied[BLUE], state.stacked);
    let blue_captures =
        Captures::for_color(&blue_reachable_fields, state.occupied[RED], state.stacked);
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
    // Check whether the other player has a winning move
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

    let red = evaluate_color(
        state,
        RED,
        &red_reachable_fields,
        //&blue_reachable_fields,
        &red_captures,
        //&blue_captures,
    );
    let blue = evaluate_color(
        state,
        BLUE,
        &blue_reachable_fields,
        //&red_reachable_fields,
        &blue_captures,
        //&red_captures,
    );
    let turn_advantage =
        DEFAULT_PARAMETERS.turn_advantage * if state.ply % 2 == 0 { 1.0 } else { -1.0 };
    (red - blue + turn_advantage).round() as i16
}

fn evaluate_color(
    state: &GameState,
    color: usize,
    my_reachable_fields: &ReachableFields,
    //opponent_reachable_fields: &ReachableFields,
    my_captures: &Captures,
    //opponent_captures: &Captures,
) -> f32 {
    let amber_value = DEFAULT_PARAMETERS.amber_value * (state.ambers[color] as f32);
    let stacked_piece_value = DEFAULT_PARAMETERS.stacked_piece_value
        * ((state.stacked & state.occupied[color]).count_ones() as f32);
    let capture_value = DEFAULT_PARAMETERS.capture_value
        * ((my_captures.captures_stack | my_captures.stack_captures).count_ones() as f32);
    let reachable_fields_value = DEFAULT_PARAMETERS.reachable_fields_value[COCKLE as usize]
        * (my_reachable_fields.cockle.count_ones() as f32)
        + DEFAULT_PARAMETERS.reachable_fields_value[GULL as usize]
            * (my_reachable_fields.gull.count_ones() as f32)
        + DEFAULT_PARAMETERS.reachable_fields_value[STARFISH as usize]
            * (my_reachable_fields.starfish.count_ones() as f32)
        + DEFAULT_PARAMETERS.reachable_fields_value[SEAL as usize]
            * (my_reachable_fields.seal.count_ones() as f32);
    // TODO: Piece values
    // TODO: More evaluation features
    amber_value + stacked_piece_value + capture_value + reachable_fields_value
}
