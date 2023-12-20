use crate::quarto::{Piece, State};
use rand::Rng;

pub fn random_action(state: &State) -> (Option<(usize, usize)>, Option<Piece>) {
    let mut rng = rand::thread_rng();
    let mut put: Option<(usize, usize)> = None;
    if !state.is_first_turn() {
        let actions = state.legal_placements();
        put = Some(*actions.get(rng.gen::<usize>() % actions.len()).unwrap());
    }
    let mut select: Option<Piece> = None;
    if put.is_none() || !state.can_put_then_win(put.unwrap().0, put.unwrap().1) {
        let actions = state.legal_pieces();
        select = Some(*actions.get(rng.gen::<usize>() % actions.len()).unwrap());
    }
    (put, select)
}
