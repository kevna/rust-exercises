mod generation;

use generation::Generation;

fn main() {
    for grid in [
        // vec![vec![false; 10]; 10],
        vec![
            vec![false, true, false],
            vec![false, false, true],
            vec![true, true, true],
        ],
        vec![
            vec![false, false, true],
            vec![true, false, true],
            vec![false, true, true],
        ],
    ] {
        let gen = Generation::new(grid);
        println!("{}", gen);
    }
    let gen = Generation::soup(25);
    println!("{}", gen);
}
