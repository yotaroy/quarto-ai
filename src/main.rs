use crate::quarto::play_game;
use crate::random::random_action;

mod quarto;
mod random;

fn main() {
    play_game(random_action, random_action);
}
