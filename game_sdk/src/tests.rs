use super::action::{ActionList, ActionListStack};
use super::gamerules;
use super::gamestate::GameState;
use rand::{rngs::SmallRng, RngCore, SeedableRng};
/*
pub fn random_gamestate(ply: u8) -> GameState {
    let mut rng = SmallRng::from_entropy();
    let mut state = GameState::random();
    let mut al = ActionList::default();
    while state.ply < ply {
        gamerules::get_legal_actions(&state, &mut al);
        let action = al[rng.next_u64() as usize % al.size];
        gamerules::do_action(&mut state, action);
    }
    state
}*/

#[test]
pub fn test_fen_conversion() {
    for _ in 0..1_000 {
        let state = GameState::random();
        assert!(state == GameState::from_fen(&state.to_fen()));
    }
}

#[test]
pub fn test_undo() {
    let mut rng = SmallRng::from_entropy();
    let mut al = ActionList::default();
    let mut history = ActionList::default();
    for _ in 0..1_000 {
        let mut state = GameState::random();
        let initial_state = state.clone();
        while !gamerules::is_game_over(&state) {
            gamerules::get_legal_actions(&state, &mut al);
            let action = al[rng.next_u64() as usize % al.size];
            gamerules::do_action(&mut state, action);
            history.push(action);
        }
        for i in 0..history.size {
            let index = history.size - i - 1;
            gamerules::undo_action(&mut state, history[index]);
        }
        history.clear();
        state.undo = initial_state.undo;
        assert!(state == initial_state);
    }
}

#[test]
pub fn test_hashing() {
    let mut rng = SmallRng::from_entropy();
    let mut al = ActionList::default();

    for _ in 0..100 {
        let mut state = GameState::random();
        while !gamerules::is_game_over(&state) {
            gamerules::get_legal_actions(&state, &mut al);
            let action = al[rng.next_u64() as usize % al.size];
            gamerules::do_action(&mut state, action);
            let hash = state.hash;
            state.recalculate_hash();
            assert_eq!(state.hash, hash);
        }
    }
}

#[test]
pub fn test_move_generation() {
    let mut als = ActionListStack::with_size(10);
    let mut state = GameState::from_fen("29 281474976710657 1099511628032 8589935104 0 35184372088832 0 549755813888 2147483776 0 2");
    let result = perft(&mut state, 6, &mut als);
    assert_eq!(result, 4961202);
}

fn perft(state: &mut GameState, depth: usize, als: &mut ActionListStack) -> usize {
    if depth == 0 {
        return 1;
    }
    let mut children: usize = 0;
    gamerules::get_legal_actions(state, &mut als[depth]);
    for i in 0..als[depth].size {
        let action = als[depth][i];
        gamerules::do_action(state, action);
        children += perft(state, depth - 1, als);
        gamerules::undo_action(state, action);
    }
    children
}

// cargo test --release -- --nocapture
