use crate::quarto::{Piece, State, WinningStatus};
use crate::random::random_action;
use std::cmp::max;

fn playout(state: &mut State) -> f64 {
    match state.get_winning_status() {
        WinningStatus::WIN => 1.0,
        WinningStatus::DRAW => 0.5,
        WinningStatus::NONE => {
            let (place, piece) = random_action(state);
            if let Some(p) = place {
                state.put_piece(p.0, p.1);
            }
            if let Some(piece) = piece {
                state.select_piece(piece);
            }
            1.0 - playout(state)
        }
        _ => panic!("unreachable error"),
    }
}

pub fn primitive_monte_carlo_action(
    state: &State,
    playout_number: usize,
) -> (Option<(usize, usize)>, Option<Piece>) {
    let mut legal_places = Vec::new();
    if !state.is_first_turn() {
        legal_places = state.legal_placements();
    }
    let mut legal_pieces = Vec::new();
    if !state.is_done() {
        legal_pieces = state.legal_pieces();
    }

    let mut values = vec![vec![0.0; max(legal_places.len(), 1)]; max(legal_pieces.len(), 1)];
    let mut cnts = vec![vec![0usize; max(legal_places.len(), 1)]; max(legal_pieces.len(), 1)];
    for cnt in 0..playout_number {
        let mut next_state = *state;
        let next_state = &mut next_state;

        if !legal_places.is_empty() {
            let place = legal_places[cnt % legal_places.len()];
            next_state.put_piece(place.0, place.1);
        };
        if !legal_pieces.is_empty() {
            let piece = legal_pieces[cnt % legal_pieces.len()];
            next_state.select_piece(piece);
        };

        values[max(cnt % legal_places.len(), 1)][max(cnt % legal_pieces.len(), 1)] +=
            1.0 - playout(next_state);
        cnts[max(cnt % legal_places.len(), 1)][max(cnt % legal_pieces.len(), 1)] += 1;
    }

    let mut best_action_idx = (usize::MAX, usize::MAX);
    let mut best_score = f64::NEG_INFINITY;
    for place_idx in 0..max(legal_places.len(), 1) {
        for piece_idx in 0..max(legal_pieces.len(), 1) {
            let value_mean = values[place_idx][piece_idx] / cnts[place_idx][piece_idx] as f64;
            if value_mean > best_score {
                best_score = value_mean;
                best_action_idx = (place_idx, piece_idx);
            }
        }
    }
    let place = if legal_places.is_empty() {
        None
    } else {
        Some(legal_places[best_action_idx.0])
    };
    let piece = if legal_pieces.is_empty() {
        None
    } else {
        Some(legal_pieces[best_action_idx.1])
    };
    (place, piece)
}
