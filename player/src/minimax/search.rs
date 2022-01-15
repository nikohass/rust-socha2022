use super::cache::{TranspositionTable, TranspositionTableEntry};
use super::evaluation::static_evaluation;
use game_sdk::action::*;
use game_sdk::gamerules;
use game_sdk::gamestate::*;
use game_sdk::player::Player;
use std::time::Instant;

pub const MATE_VALUE: i16 = 31_000;
pub const MATED_VALUE: i16 = -MATE_VALUE;
pub const MAX_VALUE: i16 = std::i16::MAX;
pub const MIN_VALUE: i16 = -MAX_VALUE;
pub const MAX_SEARCH_DEPTH: usize = 64;
pub const STANDARD_VALUE: i16 = std::i16::MIN + 1;

pub struct Searcher {
    pub root_ply: u8,
    pub stop: bool,
    pub nodes_searched: usize,
    pub als: ActionListStack,
    pub pv: ActionList,
    pub pv_table: ActionListStack,
    pub start_time: Instant,
    pub time_limit: u128,
    pub tt: TranspositionTable,
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
        //let mut best_value = MIN_VALUE;
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
            //best_value = current_value;
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
        beta: i16,
    ) -> i16 {
        self.pv_table[depth].clear();
        self.nodes_searched += 1;
        let is_pv_node = beta > 1 + alpha;
        let is_root_node = depth == 0;
        let is_game_over = gamerules::is_game_over(state);
        let original_alpha = alpha;
        let hash = state.hash as usize;
        let mut best_value = MIN_VALUE;
        let color = match state.ply % 2 {
            0 => 1,
            _ => -1,
        };
        if self.nodes_searched % 2048 == 0 {
            self.stop = self.start_time.elapsed().as_millis() >= self.time_limit;
        }

        if is_game_over {
            let result = gamerules::game_result(state);
            return (MATE_VALUE + 200 - depth as i16) * color * result;
        }
        if depth_left == 0 {
            return static_evaluation(state) * color;
        }
        if self.stop {
            return STANDARD_VALUE;
        }

        gamerules::get_legal_actions(state, &mut self.als[depth]);
        if self.als[depth].size == 0 {
            return MATE_VALUE;
        }
        if let Some(tt_entry) = self.tt.lookup(hash) {
            if tt_entry.depth >= depth_left as u8 {
                let tt_value = if tt_entry.value >= MATE_VALUE {
                    tt_entry.value - depth as i16
                } else if tt_entry.value <= MATED_VALUE {
                    tt_entry.value + depth as i16
                } else {
                    tt_entry.value
                };
                let mate_length = if tt_entry.value.abs() >= MATE_VALUE {
                    MATE_VALUE + 200 - tt_entry.value.abs()
                } else {
                    0
                };
                if state.ply + mate_length as u8 <= 60
                    && !is_root_node
                    && (!tt_entry.alpha && !tt_entry.beta
                        || tt_entry.beta && tt_value >= beta
                        || tt_entry.alpha && alpha >= tt_value)
                {
                    return tt_value;
                }
                //let draw_length = if tt_value == 0 && tt_entry.depth
            }
            let tt_action = tt_entry.action;
            for index in 0..self.als[depth].size {
                if self.als[depth][index] == tt_action {
                    self.als[depth].swap(index, 0);
                    //best_value = tt_entry.value;
                    break;
                }
            }
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
            //let value = -self.pv_search(state, depth + 1, depth_left - 1, -beta, -alpha);
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
        self.tt.insert(
            hash,
            TranspositionTableEntry {
                value: best_value,
                action: self.pv_table[depth][0],
                depth: depth as u8,
                hash,
                alpha: best_value <= original_alpha,
                beta: alpha >= beta,
            },
        );
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
            tt: TranspositionTable::default(),
        }
    }
}

impl Player for Searcher {
    fn on_move_request(&mut self, state: &GameState) -> Action {
        self.search(state)
    }

    fn set_time_limit(&mut self, time_limit: u64) {
        self.time_limit = time_limit as u128
    }
}
