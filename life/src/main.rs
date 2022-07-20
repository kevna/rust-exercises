mod generation;
mod game;
mod rule;

use structopt::StructOpt;
use generation::Generation;
use game::Game;
use rule::Rule;

#[derive(Debug, StructOpt)]
#[structopt(name="life", about="A rust implementation of John Conway's Game of Life")]
struct Opt {
    #[structopt(default_value="200")]
    width: usize,
    #[structopt(default_value="100")]
    height: usize,
}

fn main() {
    let opt = Opt::from_args();
    let mut game = Game::new(
        Generation::soup(opt.width, opt.height),
        Rule::new("B3/S23"),
    );
    game.run();
}
