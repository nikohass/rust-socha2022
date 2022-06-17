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

// TODO: History Heuristic

pub struct Searcher {
    pub root_ply: u8,
    pub stop: bool,
    pub nodes_searched: usize,
    //pub als: ActionListStack,
    pub move_orderer: MoveOrderer,
    pub pv: ActionList,
    pub pv_table: ActionListStack,
    pub pv_hash_table: Vec<usize>,
    pub killer_moves: [[Action; 2]; MAX_SEARCH_DEPTH],
    pub start_time: Instant,
    pub time_limit: u128,
    pub tt: TranspositionTable,
}

impl Default for Searcher {
    fn default() -> Self {
        Self {
            root_ply: 0,
            stop: false,
            nodes_searched: 0,
            move_orderer: MoveOrderer::default(),
            pv: ActionList::default(),
            pv_table: ActionListStack::with_size(MAX_SEARCH_DEPTH),
            pv_hash_table: Vec::with_capacity(MAX_SEARCH_DEPTH),
            killer_moves: [[Action::none(); 2]; MAX_SEARCH_DEPTH],
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

    fn reset(&mut self) {
        self.root_ply = 0;
        self.stop = false;
        self.nodes_searched = 0;
        self.pv = ActionList::default();
        self.pv_table = ActionListStack::with_size(MAX_SEARCH_DEPTH);
        self.pv_hash_table = Vec::with_capacity(MAX_SEARCH_DEPTH);
        self.tt = TranspositionTable::default();
        self.killer_moves = [[Action::none(); 2]; MAX_SEARCH_DEPTH];
        self.move_orderer = MoveOrderer::default();
    }
}

impl Searcher {
    pub fn search(&mut self, state: &GameState) -> Action {
        println!("Searching action using PV-Search for {}", state.to_fen());
        println!("Depth  Value     Nodes PV");
        let mut state = state.clone();
        self.start_time = Instant::now();
        self.nodes_searched = 0;
        self.root_ply = state.ply;
        self.stop = false;
        self.pv.clear();
        self.pv_hash_table.clear();
        let mut best_action = Action::none();
        for depth in 1..=MAX_SEARCH_DEPTH {
            let current_value = self.pv_search(&mut state, 0, depth, MIN_VALUE, MAX_VALUE);
            print!("{:5} {:6} {:9} ", depth, current_value, self.nodes_searched);
            if self.stop {
                println!("(canceled)");
                break;
            }
            let mut toy_state = state.clone();
            self.pv = self.pv_table[0].clone();
            println!("{}", self.pv);
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
        self.nodes_searched += 1;
        let is_pv_node = beta > 1 + alpha;
        //let is_root_node = depth == 0;
        let is_game_over = gamerules::is_game_over(state);
        let original_alpha = alpha;
        let hash = state.hash as usize;
        let mut best_value = STANDARD_VALUE;
        let color_sign = match state.ply % 2 {
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
            &self.killer_moves[depth],
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
                        if action != self.killer_moves[depth][0]
                            && action != self.killer_moves[depth][1]
                        {
                            self.killer_moves[depth][0] = self.killer_moves[depth][1];
                            self.killer_moves[depth][1] = action;
                        }
                        break;
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
