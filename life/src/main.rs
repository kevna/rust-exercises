mod generation;
mod game;

use generation::Generation;
use game::Game;

fn main() {
    let mut game = Game::new(
        Generation::soup(100)
    );
    game.run();
}
