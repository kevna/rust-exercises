use std::fmt;

fn add(unsigned: &usize, signed: &i32) -> usize {
    if *signed < 0 {
        return unsigned.wrapping_sub(signed.abs() as usize)
    }
    return unsigned.wrapping_add(*signed as usize)
}

type Grid = Vec<Vec<bool>>;

pub struct Generation {
    pub grid: Grid,
}

impl Generation {
    pub fn new(grid: Grid) -> Generation {
        return Generation{grid}
    }

    pub fn soup(size: usize) -> Generation {
        let mut grid = vec![vec![false; size]; size];
        let amount = (size*size)/2;
        for _ in 0..amount {
            let x = fastrand::usize(..size);
            let y = fastrand::usize(..size);
            grid[y][x] = true;
        }
        return Generation::new(grid)
    }

    pub fn alive(&self, x: &usize, y: &usize) -> bool {
        return self.grid[*y][*x];
    }

    pub fn neigbour_counts(&self) -> Vec<Vec<i8>> {
        let width = self.grid[0].len();
        let height = self.grid.len();
        let mut result = vec![vec![0; width]; height];
        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell {
                    for i in -1..2 {
                        for j in -1..2 {
                            let tmp_y = add(&y, &i);
                            let tmp_x = add(&x, &j);
                            if tmp_x < width && tmp_y < height {
                                result[add(&y, &i)][add(&x, &j)] += 1
                            }
                        }
                    }
                    result[y][x] -= 1
                }
            }
        }
        return result;
    }
}

impl fmt::Display for Generation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result: String = "".to_owned();
        for pair in self.grid.chunks(2) {
            let mut pair_two = &vec![false; pair[0].len()];
            if pair.len() >= 2 {
                pair_two = &pair[1];
            }
            for (upper, lower) in pair[0].iter().zip(pair_two) {
                if *upper && *lower {
                    result += "█"
                } else if *upper {
                    result += "▀"
                } else if *lower {
                    result += "▄"
                } else {
                    result += " "
                }
            }
            result += "\n";
        }
        return write!(f, "{}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        vec![
            vec![false, true, false],
            vec![false, false, true],
            vec![true, true, true],
        ],
        " ▀▄\n▀▀▀\n"
    )]
    #[case(
        vec![
            vec![false, false, true],
            vec![true, false, true],
            vec![false, true, true],
        ],
        "▄ █\n ▀▀\n"
    )]
    #[case(
        vec![
            vec![false, true, false],
            vec![false, true, false],
            vec![false, true, false],
        ],
        " █ \n ▀ \n"
    )]
    fn test_generation_display(#[case] grid: Grid, #[case] expected: &str) {
        let gen = Generation::new(grid);
        assert_eq!(expected, gen.to_string());
    }
}
