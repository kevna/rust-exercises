mod generation;
mod game;
mod rule;

use generation::Generation;
use game::Game;
use rule::Rule;

fn main() {
    let mut game = Game::new(
        Generation::soup(100),
        Rule::new("B3/S23"),
    );
    game.run();
}
