mod generation;
mod game;
mod rule;
mod rle;

use structopt::StructOpt;
use generation::Generation;
use game::Game;
use rule::Rule;

#[derive(Debug, StructOpt)]
#[structopt(name="life", about="A rust implementation of John Conway's Game of Life")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Subcommand,
    #[structopt(short="r", long="rules", default_value="original")]
    rules: String,
    #[structopt(long="delay", default_value="24", about="Delay between generations (in miliseconds)")]
    delay: u32,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    Soup {
        #[structopt(default_value="200")]
        width: usize,
        #[structopt(default_value="100")]
        height: usize,
        #[structopt(long="density", default_value="0.25")]
        density: f64,
    },
    File {
        filename: String,
    },
}

fn main() {
    let opt = Opt::from_args();
    let generation: Generation;
    match opt.cmd {
        Subcommand::Soup {width, height, density} => {
            generation = Generation::soup(width, height, density);
        }
        Subcommand::File {filename} => {
            let file = rle::read_file(&filename).unwrap();
            generation = Generation::new(file);
        }
    }
    let mut game = Game::new(
        generation,
        Rule::new(&opt.rules),
    );
    game.run(&opt.delay);
}
