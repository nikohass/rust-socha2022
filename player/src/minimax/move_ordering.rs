use super::search::MAX_SEARCH_DEPTH;
use game_sdk::action::Action;
use game_sdk::action::ActionListStack;
use game_sdk::action::MAX_ACTIONS;
use game_sdk::gamerules;
use game_sdk::gamestate::GameState;

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
    pub fn generate_moves(
        &mut self,
        state: &GameState,
        depth: usize,
        pv_action: Action,
        tt_action: Action,
        killer_moves: &[Action; 2],
    ) {
        gamerules::get_legal_actions(state, &mut self.als[depth]);
        for i in 0..self.als[depth].size {
            let action = self.als[depth][i];
            let mut value = if action.is_capture() { 1 } else { 0 };
            if action == pv_action {
                value += 100;
            }
            if action == tt_action {
                value += 10;
            }
            if action == killer_moves[0] || action == killer_moves[1] {
                value += 1;
            }
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
