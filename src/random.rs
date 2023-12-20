use rand::Rng;
use crate::quarto::{Piece, State};

pub fn random_action(state: &State) -> (Option<(usize, usize)>, Option<Piece>) {
    let mut rng = rand::thread_rng();
    let mut put: Option<(usize, usize)> = None;
    if !state.is_first_turn() {
        let actions = state.legal_placements();
        put = Some(*actions.get(rng.gen::<usize>() % actions.len()).unwrap());
    }
    let mut select: Option<Piece> = None;
    if !state.is_done() {
        let actions = state.legal_pieces();
        select = Some(**actions.get(rng.gen::<usize>() % actions.len()).unwrap());
    }
    (put, select)
}
