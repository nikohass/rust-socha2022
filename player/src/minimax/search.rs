use super::evaluation::static_evaluation;
use game_sdk::action::*;
use game_sdk::gamerules;
use game_sdk::gamestate::*;
use game_sdk::player::Player;
use std::time::Instant;

pub const MATE_VALUE: i16 = 32_000;
pub const MAX_VALUE: i16 = std::i16::MAX;
pub const MIN_VALUE: i16 = -MAX_VALUE;
pub const MAX_SEARCH_DEPTH: usize = 64;

pub struct Searcher {
    pub root_ply: u8,
    pub stop: bool,
    pub nodes_searched: usize,
    pub als: ActionListStack,
    pub pv: ActionList,
    pub pv_table: ActionListStack,
    pub start_time: Instant,
    pub time_limit: u128,
}

impl Searcher {
    pub fn search(&mut self, state: &GameState) -> Action {
        println!("Searching action using PV-Search for {}", state.to_fen());
        let mut state = state.clone();
        self.start_time = Instant::now();
        self.nodes_searched = 0;
        self.root_ply = state.ply;
        self.stop = false;
        self.pv.clear();
        let mut best_value = MIN_VALUE;
        let mut best_action = Action::none();
        for depth in 1..=MAX_SEARCH_DEPTH {
            let current_value = self.pv_search(&mut state, 0, depth, MIN_VALUE, MAX_VALUE);
            println!(
                "Depth: {} Score: {} Nodes: {} PV: {}",
                depth, current_value, self.nodes_searched, self.pv
            );
            if self.stop {
                break;
            }
            best_value = current_value;
            self.pv = self.pv_table[0].clone();
            best_action = self.pv[0];
        }
        best_action
    }

    fn pv_search(
        &mut self,
        state: &mut GameState,
        depth: usize,
        depth_left: usize,
        mut alpha: i16,
        mut beta: i16,
    ) -> i16 {
        self.pv_table[depth].clear();
        let is_pv_node = beta > 1 + alpha;
        let is_game_over = gamerules::is_game_over(state);
        let original_alpha = alpha;
        self.nodes_searched += 1;
        if self.nodes_searched % 2048 == 0 {
            self.stop = self.start_time.elapsed().as_millis() >= self.time_limit;
        }
        if self.stop || depth_left == 0 || is_game_over {
            return static_evaluation(state)
                * match state.ply % 2 {
                    0 => 1,
                    _ => -1,
                };
        }
        let mut best_value = MIN_VALUE;
        gamerules::get_legal_actions(state, &mut self.als[depth]);
        if self.als[depth].size == 0 {
            return MATE_VALUE;
        }
        for index in 0..self.als[depth].size {
            let action = self.als[depth][index];
            gamerules::do_action(state, action);
            let value = if index == 0 {
                -self.pv_search(state, depth + 1, depth_left - 1, -beta, -alpha)
            } else {
                let mut value =
                    -self.pv_search(state, depth + 1, depth_left - 1, -alpha - 1, -alpha);
                if value > alpha {
                    value = -self.pv_search(state, depth + 1, depth_left - 1, -beta, -alpha);
                }
                value
            };
            gamerules::undo_action(state, action);
            if value > best_value {
                best_value = value;
                self.pv_table[depth].clear();
                self.pv_table[depth].push(action);
                if is_pv_node {
                    for i in 0..self.pv_table[depth + 1].size {
                        let action = self.pv_table[depth + 1][i];
                        self.pv_table[depth].push(action);
                    }
                }
                if value > alpha {
                    alpha = value;
                }
            }
            if alpha >= beta {
                break;
            }
        }
        alpha
    }
}

impl Default for Searcher {
    fn default() -> Self {
        Self {
            root_ply: 0,
            stop: false,
            nodes_searched: 0,
            als: ActionListStack::with_size(MAX_SEARCH_DEPTH),
            pv: ActionList::default(),
            pv_table: ActionListStack::with_size(MAX_SEARCH_DEPTH),
            start_time: Instant::now(),
            time_limit: 1980,
        }
    }
}

impl Player for Searcher {
    fn on_move_request(&mut self, state: &GameState) -> Action {
        self.search(state)
    }
}
