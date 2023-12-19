use crate::quarto::State;

mod quarto;

fn main() {
    let game = State::new();
    game.print();
}
