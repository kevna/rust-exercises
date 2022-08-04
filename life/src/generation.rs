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

    pub fn soup(width: usize, height: usize, density: f64) -> Generation {
        let mut grid = vec![vec![false; width]; height];
        let amount = ((width*height) as f64 *density).ceil() as i32;
        for _ in 0..amount {
            let x = fastrand::usize(..width);
            let y = fastrand::usize(..height);
            grid[y][x] = true;
        }
        return Generation::new(grid)
    }

    pub fn alive(&self, x: &usize, y: &usize) -> bool {
        return self.grid[*y][*x];
    }

    pub fn neigbour_counts(&self, neighbourhood: &Vec<(i32, i32)>) -> Vec<Vec<i8>> {
        let width = self.grid[0].len();
        let height = self.grid.len();
        let mut result = vec![vec![0; width]; height];
        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell {
                    for (i, j) in neighbourhood {
                        let tmp_y = add(&y, &j);
                        let tmp_x = add(&x, &i);
                        if tmp_x < width && tmp_y < height {
                            result[tmp_y][tmp_x] += 1
                        }
                    }
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
        return write!(f, "{}", &result[..result.len()-1]);
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
        " ▀▄\n▀▀▀"
    )]
    #[case(
        vec![
            vec![false, false, true],
            vec![true, false, true],
            vec![false, true, true],
        ],
        "▄ █\n ▀▀"
    )]
    #[case(
        vec![
            vec![false, true, false],
            vec![false, true, false],
            vec![false, true, false],
        ],
        " █ \n ▀ "
    )]
    fn test_generation_display(#[case] grid: Grid, #[case] expected: &str) {
        let gen = Generation::new(grid);
        assert_eq!(expected, gen.to_string());
    }
}
