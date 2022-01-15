use super::action::Action;
use super::gamestate::GameState;

pub trait Player {
    fn on_move_request(&mut self, state: &GameState) -> Action;

    fn reset(&mut self) {}

    fn set_time_limit(&mut self, _: u64) {}
}
