use super::cache::{TranspositionTable, TranspositionTableEntry};
use super::evaluation::{static_evaluation, MATE_VALUE};
use super::move_ordering::MoveOrderer;
use game_sdk::action::*;
use game_sdk::gamerules;
use game_sdk::gamestate::*;
use game_sdk::player::Player;
use std::time::Instant;

pub const MAX_VALUE: i16 = std::i16::MAX;
pub const MIN_VALUE: i16 = -MAX_VALUE;
pub const MAX_SEARCH_DEPTH: usize = 60;
pub const STANDARD_VALUE: i16 = std::i16::MIN + 1;

pub struct Searcher {
    pub stop: bool,
    pub nodes_searched: usize,
    pub move_orderer: MoveOrderer,
    pub pv: ActionList,
    pub pv_table: ActionListStack,
    pub pv_hash_table: Vec<usize>,
    pub history_heuristic: [[[u64; 64]; 64]; 2],
    pub butterfly_heuristic: [[[u64; 64]; 64]; 2],
    pub killer_heuristic: [[Action; 2]; MAX_SEARCH_DEPTH],
    //pub counter_move_heuristic: [[Action; 64]; 64],
    pub start_time: Instant,
    pub time_limit: u128,
    pub tt: TranspositionTable,
    //pub evaluation_cache: EvaluationCache,
}

impl Default for Searcher {
    fn default() -> Self {
        Self {
            stop: false,
            nodes_searched: 0,
            move_orderer: MoveOrderer::default(),
            pv: ActionList::default(),
            pv_table: ActionListStack::with_size(MAX_SEARCH_DEPTH),
            pv_hash_table: Vec::with_capacity(MAX_SEARCH_DEPTH),
            history_heuristic: [[[0; 64]; 64]; 2],
            butterfly_heuristic: [[[1; 64]; 64]; 2],
            killer_heuristic: [[Action::none(); 2]; MAX_SEARCH_DEPTH],
            //counter_move_heuristic: [[Action::none(); 64]; 64],
            start_time: Instant::now(),
            time_limit: 1970,
            tt: TranspositionTable::default(),
            //evaluation_cache: EvaluationCache::default(),
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

    fn reset(&mut self) {
        self.stop = false;
        self.nodes_searched = 0;
        self.move_orderer = MoveOrderer::default();
        self.pv = ActionList::default();
        self.pv_table = ActionListStack::with_size(MAX_SEARCH_DEPTH);
        self.pv_hash_table = Vec::with_capacity(MAX_SEARCH_DEPTH);
        self.history_heuristic = [[[0; 64]; 64]; 2];
        self.butterfly_heuristic = [[[1; 64]; 64]; 2];
        self.killer_heuristic = [[Action::none(); 2]; MAX_SEARCH_DEPTH];
        //self.counter_move_heuristic = [[Action::none(); 64]; 64];
        self.tt = TranspositionTable::default();
        //self.evaluation_cache = EvaluationCache::default();
    }
}

impl Searcher {
    pub fn search(&mut self, state: &GameState) -> Action {
        println!("Searching action using PV-Search for {}", state.to_fen());
        println!("Depth  Value     Nodes     Elapsed   Nodes/s PV");
        let mut state = state.clone();
        self.start_time = Instant::now();
        self.nodes_searched = 0;
        self.stop = false;
        self.pv.clear();
        self.pv_hash_table.clear();
        for i in 0..2 {
            for j in 0..64 {
                for k in 0..64 {
                    self.history_heuristic[i][j][k] /= 8;
                    self.butterfly_heuristic[i][j][k] =
                        (self.butterfly_heuristic[i][j][k] / 8).max(1);
                }
            }
        }
        let mut best_action = Action::none();
        for depth in 1..=MAX_SEARCH_DEPTH {
            let current_value = self.pv_search(&mut state, 0, depth, MIN_VALUE, MAX_VALUE);
            let elapsed = Instant::now().duration_since(self.start_time).as_micros();
            let nps = self.nodes_searched as f64 / (elapsed as f64 / 1_000_000.0);
            print!(
                "{:5} {:6} {:9} {:9}μs {:9.0} ",
                depth, current_value, self.nodes_searched, elapsed, nps
            );
            if self.stop {
                println!("(canceled)");
                break;
            }
            let mut toy_state = state.clone();
            self.pv = self.pv_table[0].clone();
            if self.pv.size != 0 {
                best_action = self.pv[0];
            }
            println!("{}", self.format_pv());
            if self.pv.size != depth {
                println!("Reached the end of the search tree.");
                if current_value >= MATE_VALUE {
                    println!(
                        "Mate in {}. Value: +{}",
                        depth - 1,
                        current_value - MATE_VALUE
                    );
                } else if current_value == 0 {
                    println!("Draw in {}", depth - 1);
                } else {
                    println!(
                        "Mated in {} Value: {}",
                        depth - 1,
                        current_value + MATE_VALUE
                    );
                }
                break;
            }
            self.pv_hash_table.clear();
            for i in 0..self.pv.size {
                self.pv_hash_table.push(toy_state.hash as usize);
                gamerules::do_action(&mut toy_state, self.pv[i]);
            }
        }
        if best_action == Action::none() {
            gamerules::get_legal_actions(&state, &mut self.pv);
            best_action = self.pv[0];
            println!("No move found.");
        }
        best_action
    }

    fn format_pv(&self) -> String {
        let mut s = String::new();
        let mut line_length = 0;
        for i in 0..self.pv.size {
            let next_action = &format!("{} ", self.pv[i]);
            let len = next_action.len();
            if line_length + len > 100 {
                s.push_str("\n    .      .         .           .         . ");
                line_length = 0;
            }
            line_length += len;
            s.push_str(next_action);
        }
        s
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
        self.nodes_searched += 1;
        let is_pv_node = beta > 1 + alpha;
        //let is_root_node = depth == 0;
        let is_game_over = gamerules::is_game_over(state);
        let original_alpha = alpha;
        let hash = state.hash as usize;
        let mut best_value = STANDARD_VALUE;
        let color = (state.ply % 2) as usize;
        let color_sign = match color {
            0 => 1,
            _ => -1,
        };
        if self.nodes_searched % 2048 == 0 {
            self.stop = self.start_time.elapsed().as_millis() >= self.time_limit;
        }

        if is_game_over {
            let result = gamerules::game_result(state);
            return (MATE_VALUE + 60 - depth as i16) * color_sign * result;
        }

        // Mate distance pruning
        {
            alpha = alpha.max(-(MATE_VALUE + 60 - depth as i16 - 1));
            beta = beta.min(MATE_VALUE + 60 - depth as i16 - 1);
            if alpha >= beta {
                return beta;
            }
        }

        if depth_left == 0 || self.stop {
            /*return if let Some(value) = self.evaluation_cache.lookup(hash) {
                value
            } else {
                let value = static_evaluation(state);
                self.evaluation_cache.insert(hash, value);
                value
            } * color_sign;*/
            return static_evaluation(state) * color_sign;
        }

        let pv_action = if self.pv_table[depth].size > 0 && hash == self.pv_hash_table[depth] {
            self.pv[depth]
        } else {
            Action::none()
        };

        let tt_action = if let Some(entry) = self.tt.lookup(hash) {
            // TODO:
            entry.action
        } else {
            Action::none()
        };

        self.move_orderer.generate_moves(
            state,
            depth,
            pv_action,
            tt_action,
            &self.killer_heuristic[depth],
            &self.history_heuristic[color],
            &self.butterfly_heuristic[color],
        );
        if self.move_orderer.als[depth].size == 0 {
            return MATE_VALUE;
        }

        let mut is_first = true;
        loop {
            let action = self.move_orderer.next(depth);
            if action == Action::none() {
                break;
            }
            gamerules::do_action(state, action);
            let value = if is_first {
                is_first = false;
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
                    if alpha >= beta {
                        self.history_heuristic[color][action.from() as usize]
                            [action.to() as usize] += (depth_left as u64) * (depth_left as u64);
                        if action != self.killer_heuristic[depth][0]
                            && action != self.killer_heuristic[depth][1]
                        {
                            self.killer_heuristic[depth][0] = self.killer_heuristic[depth][1];
                            self.killer_heuristic[depth][1] = action;
                        }
                        break;
                    } else {
                        self.butterfly_heuristic[color][action.from() as usize]
                            [action.to() as usize] += depth_left as u64;
                    }
                }
            }
        }
        if !self.stop {
            self.tt.insert(
                hash,
                TranspositionTableEntry {
                    value: best_value,
                    action: self.pv_table[depth][0],
                    depth: depth_left as u8,
                    hash,
                    alpha: best_value <= original_alpha,
                    beta: alpha >= beta,
                },
            );
        }
        alpha
    }
}
