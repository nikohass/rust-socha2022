use game_sdk::gamerules::*;
use game_sdk::gamestate::*;
use game_sdk::piece::*;

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

struct EvaluationParameters {
    amber_value: f32,
}

pub const DEFAULT_PARAMETERS: EvaluationParameters = EvaluationParameters { amber_value: 3.0 };

impl ReachableFields {
    pub fn for_color(color: Color, board: &[u64; 4], stacked: u64) -> Self {
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
        self.all = self.cockle | self.gull | self.starfish | self.seal;
        self.all_stacked =
            self.cockle_stack | self.gull_stack | self.starfish_stack | self.seal_stack;
        reachable_fields
    }
}

pub fn save_capture(
    current_player_reachable_fields: ReachableFields,
    current_player_occupied_fields: u64,
    other_player_reachable_fields: ReachableFields,
    other_player_occupied_fields: u64,
    stacked: u64,
) -> bool {
    let stacked_pieces_that_i_can_capture =
        current_player_reachable_fields.all & other_player_occupied_fields & stacked;
    let pieces_that_i_can_capture_with_a_stacked_piece =
        current_player_reachable_fields.all_stacked & other_player_occupied_fields;
    stacked_pieces_that_i_can_capture | pieces_that_i_can_capture_with_a_stacked_piece > 0
}

pub fn evaluate_state(state: &GameState) -> i16 {
    let parameters = DEFAULT_PARAMETERS;
    let red_reachable_fields =
        ReachableFields::for_color(Color::Red, &state.board[RED as usize], state.stacked);
    let blue_reachable_fields =
        ReachableFields::for_color(Color::Blue, &state.board[BLUE as usize], state.stacked);
    let red_turn = state.ply % 2 == 0;

    // Check whether the current player has a winning move
    if red_turn {
        if state.ambers[Color::RED as usize] == 1 {
            if save_capture(
                red_reachable_fields,
                state.occupied_fields[RED as usize],
                blue_reachable_fields,
                state.occupied_fields[BLUE as usize],
                state.stacked,
            ) {
                // Its reds turn
                // Red already has an amber
                // Red can get an amber in the next turn
                // => Red wins
                return MATE_VALUE;
            }
        }
    } else {
        if state.ambers[Color::BLUE as usize] == 1 {
            if save_capture(
                blue_reachable_fields,
                state.occupied_fields[BLUE as usize],
                red_reachable_fields,
                state.occupied_fields[RED as usize],
                state.stacked,
            ) {
                // Its blues turn
                // Blue already has an amber
                // Blue can get an amber in the next turn
                // => Blue wins
                return -MATE_VALUE;
            }
        }
    }
    0
}

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
