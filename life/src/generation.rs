use std::fmt;

type Grid = Vec<Vec<bool>>;

pub struct Generation {
    grid: Grid,
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
