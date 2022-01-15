use super::minimax::evaluation::static_evaluation;
use game_sdk::action::{Action, ActionList};
use game_sdk::gamerules;
use game_sdk::gamestate::{GameState, BLUE, RED};
use game_sdk::player::Player;

pub struct GreedyPlayer {
    al: ActionList,
}

impl GreedyPlayer {
    pub fn get_action(&mut self, state: &GameState) -> Action {
        let mut state = state.clone();
        gamerules::get_legal_actions(&state, &mut self.al);
        let color: i16 = match state.get_current_color() {
            RED => -1,
            BLUE => 1,
            _ => panic!(),
        };
        let mut best_action = Action::none();
        let mut best_value = std::i16::MIN;
        for i in 0..self.al.size {
            let action = self.al[i];
            gamerules::do_action(&mut state, action);
            let value = static_evaluation(&state) * color;
            gamerules::undo_action(&mut state, action);
            if value > best_value {
                best_value = value;
                best_action = action;
            }
        }
        best_action
    }
}

impl Player for GreedyPlayer {
    fn on_move_request(&mut self, state: &GameState) -> Action {
        self.get_action(state)
    }
}

impl Default for GreedyPlayer {
    fn default() -> Self {
        Self {
            al: ActionList::default(),
        }
    }
}
