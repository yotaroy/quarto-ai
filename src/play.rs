use crate::quarto::{Piece, State, WinningStatus};

pub type ActionFn = fn(state: &State) -> (Option<(usize, usize)>, Option<Piece>);

pub fn play_game(player_1_action_fn: ActionFn, player_2_action_fn: ActionFn) {
    let mut state = State::new();
    state.print();

    while !state.is_done() {
        {
            println!("1p ----------------------------------------");

            println!("action:");
            let (action, piece) = player_1_action_fn(&state);
            if let Some((h, w)) = action {
                state.put_piece(h, w);
                println!("\tput: ({}, {})", h, w);
            }
            if state.is_done() {
                break;
            }
            if let Some(piece) = piece {
                state.select_piece(piece);
                println!("\tselect: {}", piece);
            }
            println!();
            state.print();
        }
        {
            println!("2p ----------------------------------------");

            println!("action:");
            let (action, piece) = player_2_action_fn(&state);
            if let Some((h, w)) = action {
                state.put_piece(h, w);
                println!("\tput: ({}, {})", h, w);
            }
            if state.is_done() {
                break;
            }
            if let Some(piece) = piece {
                state.select_piece(piece);
                println!("\tselect: {}", piece);
            }
            println!();
            state.print();
        }
    }
    println!();
    state.print();

    match state.get_winning_status() {
        WinningStatus::WIN => println!(
            "winner: {}",
            if state.is_first_player() { "1p" } else { "2p" }
        ),
        WinningStatus::DRAW => println!("DRAW"),
        _ => panic!("unreachable code"),
    }
}
