use super::search::MAX_SEARCH_DEPTH;
use game_sdk::action::Action;
use game_sdk::action::ActionListStack;
use game_sdk::action::MAX_ACTIONS;
use game_sdk::gamerules;
use game_sdk::gamestate::GameState;

pub const PV_ACTION_VALUE: u64 = std::u64::MAX;
pub const TT_ACTION_VALUE: u64 = std::u64::MAX - 1;
pub const KILLER_MOVE_VALUE: u64 = std::u64::MAX - 2;
pub const CAPTURE_VALUE: u64 = 1_000;

pub struct MoveOrderer {
    pub als: ActionListStack,
    values: [[u64; MAX_ACTIONS]; MAX_SEARCH_DEPTH],
}

impl Default for MoveOrderer {
    fn default() -> Self {
        MoveOrderer {
            als: ActionListStack::with_size(MAX_SEARCH_DEPTH),
            values: [[0; MAX_ACTIONS]; MAX_SEARCH_DEPTH],
        }
    }
}

impl MoveOrderer {
    #[allow(clippy::too_many_arguments)]
    pub fn generate_moves(
        &mut self,
        state: &GameState,
        depth: usize,
        pv_action: Action,
        tt_action: Action,
        killer_heuristic: &[Action; 2],
        history_heuristic: &[[u64; 64]; 64],
        butterfly_heuristic: &[[u64; 64]; 64],
    ) {
        gamerules::get_legal_actions(state, &mut self.als[depth]);
        for i in 0..self.als[depth].size {
            let action = self.als[depth][i];
            let value = if action == pv_action {
                PV_ACTION_VALUE
            } else if action == tt_action {
                TT_ACTION_VALUE
            } else if action == killer_heuristic[0] || action == killer_heuristic[1] {
                KILLER_MOVE_VALUE
            } else {
                let history_value = history_heuristic[action.to() as usize][action.from() as usize];
                let butterfly_value =
                    butterfly_heuristic[action.to() as usize][action.from() as usize];
                let capture_value = if action.is_capture() {
                    CAPTURE_VALUE
                } else {
                    0
                };
                capture_value + (history_value as f32 / butterfly_value as f32) as u64
            };
            self.values[depth][i] = value;
        }
    }

    pub fn next(&mut self, depth: usize) -> Action {
        if self.als[depth].size == 0 {
            return Action::none();
        }
        let mut best_index = 0;
        let mut best_value = 0;
        for i in 0..self.als[depth].size {
            if self.values[depth][i] > best_value {
                best_value = self.values[depth][i];
                best_index = i;
            }
        }
        let next_action = self.als[depth][best_index];
        self.remove_action(depth, best_index);
        next_action
    }

    pub fn remove_action(&mut self, depth: usize, index: usize) {
        if self.als[depth].size == 1 {
            self.als[depth].size = 0;
            return;
        }
        let last_element = self.als[depth].size - 1;
        self.als[depth].swap(index, last_element);
        self.als[depth].size -= 1;
        self.values[depth][index] = self.values[depth][last_element];
    }
}
