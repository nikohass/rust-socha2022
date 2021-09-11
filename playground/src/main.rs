//use game_sdk::action::*;
use game_sdk::gamerules;
use game_sdk::gamestate::*;
use player::random::*;
/*
fn r(state: &mut GameState, depth_left: usize, als: &mut ActionListStack) {
    if depth_left == 0 {
        return;
    }
    gamerules::get_legal_actions(state, &mut als[depth_left]);
    for i in 0..als[depth_left].size {
        let action = als[depth_left][i];
        //let clone = state.clone();
        gamerules::do_action(state, action);
        r(state, depth_left - 1, als);
        gamerules::undo_action(state, action);
    }
}*/

fn playout(state: &mut GameState, player: &mut RandomPlayer) {
    if gamerules::is_game_over(state) {
        return;
    }
    let action = player.get_action(state);
    gamerules::do_action(state, action);
    /*println!("{}", state);
    println!("{}", bitboard_to_string(state.occupied[state.get_current_color()]));
    println!("{}", bitboard_to_string(state.stacked));
    */
    playout(state, player);
    //println!("{}", state);
    gamerules::undo_action(state, action);
}

fn main() {
    let mut player = RandomPlayer::default();
    //println!("{}", state);
    let mut i = 0;
    let mut state = GameState::random();
    loop {
        let clone = state.clone();
        i += 1;
        //println!("==========================");
        playout(&mut state, &mut player);
        if state.occupied != clone.occupied {
            println!("Occupied");
            break;
        }
        if state.board != clone.board {
            println!("board");
            break;
        }
        if state.stacked != clone.stacked {
            println!("stacked");
            break;
        }
        if i % 1024 == 0 {
            println!("{}", i);
        }
        state = GameState::random();
    }
    println!("{}", i);
    println!("{}", state);
    /*//let clone = state.clone();
    let mut als = ActionListStack::with_size(20);
    r(&mut state, 7, &mut als);
    println!("{}", state);
    */
}
