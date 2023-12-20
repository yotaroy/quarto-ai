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

pub fn human_action(state: &State) -> (Option<(usize, usize)>, Option<Piece>) {
    let mut put: Option<(usize, usize)> = None;
    if !state.is_first_turn() {
        loop {
            println!("Input put action: (h, w)");
            println!(
                "Example\t: input: {} {}",
                state.legal_placements()[0].0,
                state.legal_placements()[0].1
            );
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let h = match iter.next() {
                Some(x) => x.parse::<usize>().or(Err(())),
                None => Err(()),
            };
            let w = match iter.next() {
                Some(x) => x.parse::<usize>().or(Err(())),
                None => Err(()),
            };
            if h.is_err() || w.is_err() {
                println!("input error");
                continue;
            }
            let h = h.unwrap();
            let w = w.unwrap();
            if state.legal_placements().contains(&(h, w)) {
                put = Some((h, w));
                break;
            } else {
                println!("illegal action");
            }
        }
    }
    let mut select: Option<Piece> = None;
    if (put.is_none() || !state.can_put_then_win(put.unwrap().0, put.unwrap().1))
        && !state.is_last_turn()
    {
        loop {
            println!("Input select action: (piece)");
            println!("Example\t: input: {}", state.legal_pieces()[0].to_string());
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let piece = input.trim().parse::<Piece>();
            if piece.is_err() {
                println!("input error");
                continue;
            }
            let piece = piece.unwrap();
            if state.legal_pieces().contains(&&piece) {
                select = Some(piece);
                break;
            } else {
                println!("illegal action");
            }
        }
    }
    (put, select)
}

pub fn test_first_player_win_rate(
    game_number: i32,
    ai_pairs: ((&str, ActionFn), (&str, ActionFn)),
) {
    let mut first_player_win_rate = 0.0;
    for i in 0..game_number {
        let base_state = State::new();
        for j in 0..2 {
            let mut state = base_state;
            let first_ai;
            let second_ai;
            if j == 0 {
                first_ai = ai_pairs.0;
                second_ai = ai_pairs.1
            } else {
                first_ai = ai_pairs.1;
                second_ai = ai_pairs.0
            }
            let mut active_ai = first_ai;
            loop {
                let (action, piece) = active_ai.1(&state);
                if let Some((h, w)) = action {
                    state.put_piece(h, w);
                }
                if state.is_done() {
                    match state.get_winning_status() {
                        WinningStatus::WIN => println!("{} win!!", active_ai.0),
                        WinningStatus::DRAW => println!("draw"),
                        _ => panic!("unreachable code"),
                    }
                    break;
                }
                if let Some(piece) = piece {
                    state.select_piece(piece);
                }
                active_ai = if active_ai == first_ai {
                    second_ai
                } else {
                    first_ai
                };
            }

            let mut win_rate_point = state.get_first_player_score_for_win_rate();
            if j == 1 {
                win_rate_point = 1.0 - win_rate_point;
            }
            first_player_win_rate += win_rate_point;

            state.print();
        }
        println!(
            "i {}, w {}",
            i,
            first_player_win_rate / ((i + 1) * 2) as f64
        );
    }
    first_player_win_rate /= (game_number * 2) as f64;
    println!(
        "Winning rate of {} to {}:\t{}",
        ai_pairs.0 .0, ai_pairs.1 .0, first_player_win_rate
    );
}
