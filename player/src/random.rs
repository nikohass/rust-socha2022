use game_sdk::action::{Action, ActionList};
use game_sdk::gamerules::get_legal_actions;
use game_sdk::gamestate::GameState;
use game_sdk::player::Player;
use rand::{rngs::SmallRng, RngCore, SeedableRng};

pub struct RandomPlayer {
    rng: SmallRng,
    al: ActionList,
}

impl RandomPlayer {
    pub fn get_action(&mut self, state: &GameState) -> Action {
        get_legal_actions(state, &mut self.al);
        self.al[self.rng.next_u64() as usize % self.al.size]
    }
}

impl Default for RandomPlayer {
    fn default() -> Self {
        Self {
            rng: SmallRng::from_entropy(),
            al: ActionList::default(),
        }
    }
}

impl Player for RandomPlayer {
    fn on_move_request(&mut self, state: &GameState) -> Action {
        self.get_action(state)
    }
}
