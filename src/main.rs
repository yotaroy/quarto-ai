use crate::random::random_action;
use play::play_game;

mod play;
mod quarto;
mod random;

fn main() {
    play_game(random_action, random_action);
}
