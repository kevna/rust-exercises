use crate::generation::Generation;
use std::{thread, time};

pub struct Game {
    pub current_generation: Generation,
}

impl Game {
    pub fn new(current_generation: Generation) -> Game {
        return Game{current_generation}
    }

    pub fn next_generation(&self) -> Generation {
        let neigbours = self.current_generation.neigbour_counts();
        let mut grid = vec![vec![false; neigbours[0].len()]; neigbours.len()];

        for (y, row) in neigbours.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == 3 || (self.current_generation.alive(&x, &y) && *cell == 2) {
                    grid[y][x] = true;
                }
            }
        }
        return Generation::new(grid);
    }

    pub fn step(&mut self) {
        self.current_generation = self.next_generation();
    }

    pub fn display_grid(&self) {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            print!("{}", self.current_generation);
    }

    pub fn run(&mut self) {
        self.display_grid();
        loop {
            thread::sleep(time::Duration::new(0, 24_550_000));
            self.step();
            self.display_grid();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    type Grid = Vec<Vec<bool>>;

    #[rstest]
    #[case(
        vec![
            vec![false, true, false],
            vec![false, true, false],
            vec![false, true, false],
        ],
        vec![
            vec![false, false, false],
            vec![true, true, true],
            vec![false, false, false],
        ],
    )]
    #[case(
        vec![
            vec![false, false, false, false],
            vec![false, true, true, false],
            vec![false, true, true, false],
            vec![false, false, false, false],
        ],
        vec![
            vec![false, false, false, false],
            vec![false, true, true, false],
            vec![false, true, true, false],
            vec![false, false, false, false],
        ],
    )]
    fn test_game_next_generation(#[case] grid: Grid, #[case] expected: Grid) {
        let game = Game::new(
            Generation::new(grid)
        );
        let gen = game.next_generation();
        assert_eq!(expected, gen.grid);
    }
}
