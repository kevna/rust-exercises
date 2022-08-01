use crate::generation::Generation;
use crate::rule::Rule;
use std::{
    thread,
    time::{Duration, Instant},
};

pub struct Game {
    current_generation: Generation,
    rule: Rule,
}

impl Game {
    pub fn new(current_generation: Generation, rule: Option<Rule>) -> Game {
        let rule = rule.unwrap_or(Rule::default());
        Game{current_generation, rule}
    }

    pub fn next_generation(&self) -> Generation {
        let neigbours = self.current_generation.neigbour_counts();
        let mut grid = vec![vec![false; neigbours[0].len()]; neigbours.len()];

        for (y, row) in neigbours.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if self.rule.apply(&self.current_generation.alive(&x, &y), &(*cell as u32)) {
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
        let display = self.current_generation.to_string();
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        print!("{}", display);
    }

    pub fn run(&mut self, delay: &u32) {
        let delay = Duration::new(0, delay*1_000_000);
        let mut elapsed = Duration::new(0, 0);
        self.display_grid();
        loop {
            if delay > elapsed {
                thread::sleep(delay - elapsed);
            }
            let now = Instant::now();
            self.step();
            self.display_grid();
            elapsed = now.elapsed();
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
            Generation::new(grid),
            None,
        );
        let gen = game.next_generation();
        assert_eq!(expected, gen.grid);
    }
}
