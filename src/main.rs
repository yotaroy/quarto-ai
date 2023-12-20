use crate::montecarlo::primitive_monte_carlo_action;
use crate::play::test_first_player_win_rate;
use crate::quarto::{Piece, State};
use crate::random::random_action;

mod montecarlo;
mod play;
mod quarto;
mod random;

fn main() {
    // play_game(random_action, random_action);
    // play_game(human_action, random_action);
    test_first_player_win_rate(
        100,
        (
            (
                "primitiveMonteCarloAction 1000",
                |state: &State| -> (Option<(usize, usize)>, Option<Piece>) {
                    primitive_monte_carlo_action(state, 1000)
                },
            ),
            ("randomAI", random_action),
        ),
    )
}
