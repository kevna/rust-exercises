pub type Neighbourhood = Vec<(i32, i32)>;

fn safe_push(vector: &mut Neighbourhood, x: i32, y: i32) {
    if !(x == 0 && y == 0) {
        vector.push((x, y))
    }
}

#[derive(Debug, PartialEq)]
pub enum Pattern {
    Moore,
    VonNeumann,
}

impl Pattern {
    pub fn generate(&self, range: i32) -> Neighbourhood {
        let max = range + 1;
        let mut result = Vec::new();
        match self {
            Self::Moore => {
                for y in -range..max {
                    for x in -range..max {
                        safe_push(&mut result, x, y);
                    }
                }
            }
            Self::VonNeumann => {
                for y in -range..max {
                    let range_x = range - y.abs();
                    for x in -range_x..(range_x+1) {
                        safe_push(&mut result, x, y);
                    }
                }
            }
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Pattern::Moore, 1, vec![
        (-1, -1), (0, -1), ( 1, -1),
        (-1,  0),          ( 1,  0),
        (-1,  1), (0,  1), ( 1,  1),
    ])]
    #[case(Pattern::Moore, 3, vec![
        (-3, -3), (-2, -3), (-1, -3), (0, -3), ( 1, -3), ( 2, -3), ( 3, -3),
        (-3, -2), (-2, -2), (-1, -2), (0, -2), ( 1, -2), ( 2, -2), ( 3, -2),
        (-3, -1), (-2, -1), (-1, -1), (0, -1), ( 1, -1), ( 2, -1), ( 3, -1),
        (-3,  0), (-2,  0), (-1,  0),          ( 1,  0), ( 2,  0), ( 3,  0),
        (-3,  1), (-2,  1), (-1,  1), (0,  1), ( 1,  1), ( 2,  1), ( 3,  1),
        (-3,  2), (-2,  2), (-1,  2), (0,  2), ( 1,  2), ( 2,  2), ( 3,  2),
        (-3,  3), (-2,  3), (-1,  3), (0,  3), ( 1,  3), ( 2,  3), ( 3,  3),
    ])]
    #[case(Pattern::VonNeumann, 1, vec![
                  (0, -1),
        (-1,  0),          ( 1,  0),
                  (0,  1),
    ])]
    #[case(Pattern::VonNeumann, 3, vec![
                                      (0, -3),
                            (-1, -2), (0, -2), ( 1, -2),
                  (-2, -1), (-1, -1), (0, -1), ( 1, -1), ( 2, -1),
        (-3,  0), (-2,  0), (-1,  0),          ( 1,  0), ( 2,  0), ( 3,  0),
                  (-2,  1), (-1,  1), (0,  1), ( 1,  1), ( 2,  1),
                            (-1,  2), (0,  2), ( 1,  2),
                                      (0,  3),
    ])]
    fn test_generate_neighbourhood_ok(#[case] pattern: Pattern, #[case] range: i32, #[case] expected: Neighbourhood) {
        let actual = pattern.generate(range);
        assert_eq!(expected, actual);
    }
}
