use std::fs;
use std::num::ParseIntError;

type Grid = Vec<Vec<bool>>;

#[derive(Debug, PartialEq)]
struct Header {
    width: usize,
    height: usize,
    rule: Option<String>,
}

fn parse_header(header: &str) -> Result<Header, ParseIntError> {
    let mut result = Header{
        width: 0,
        height: 0,
        rule: None,
    };
    for row in header.split(',') {
        let (name, value) = row.split_once('=').unwrap();
        match name.trim() {
            "x" => {
                result.width = value.trim().parse()?;
            }
            "y" => {
                result.height = value.trim().parse()?;
            }
            "rule" => {
                result.rule = Some(value.trim().to_owned());
            }
            _ => {}
        }
    }
    return Ok(result);
}

fn parse_grid(header: Header, contents: &str) -> Result<Grid, ParseIntError> {
    let mut grid = vec![vec![false; header.width]; header.height];
    let mut x = 0;
    let mut y = 0;
    let mut accumulator = "".to_owned();
    for ch in contents.chars() {
        match ch {
            'o' => {
                let mut len = 1;
                if !accumulator.is_empty() {
                    len = accumulator.parse()?;
                }
                for _ in 0..len {
                    grid[y][x] = true;
                    x += 1;
                }
                accumulator = "".to_owned();
            }
            'b' => {
                let mut len = 1;
                if !accumulator.is_empty() {
                    len = accumulator.parse()?;
                }
                x += len;
                accumulator = "".to_owned();
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'  => {
                accumulator.push(ch)
            }
            '$' => {
                x = 0;
                y += 1;
            }
            _ => {}
        }
    }
    return Ok(grid);
}

pub fn parse_file(contents: &str) -> Result<Grid, ParseIntError> {
    let (header, contents) = contents.split_once("\n").unwrap();
    let dimension = parse_header(&header)?;
    return Ok(parse_grid(dimension, &contents)?);
}

pub fn read_file(filename: &str) -> Result<Grid, ParseIntError> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    return Ok(parse_file(&contents)?);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("x = 20, y = 10", Header{width: 20, height: 10, rule: None})]
    #[case("x = 20, y = 10, rule = B3/S23", Header{width: 20, height: 10, rule: Some("B3/S23".to_owned())})]
    fn test_parse_headers(#[case] header: &str, #[case] expected: Header) {
        let actual = parse_header(header).unwrap();
        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(
        Header{width: 3, height: 3, rule: None},
        "bo$2bo$3o!",
        vec![
            vec![false, true, false],
            vec![false, false, true],
            vec![true, true, true],
        ]
    )]
    fn test_parse_grid(#[case] dimension: Header, #[case] contents: &str, #[case] expected: Grid) {
        let actual = parse_grid(dimension, contents).unwrap();
        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case("x = 36, y = 9, rule = B3/S23
24bo$22bobo$12b2o6b2o12b2o$11bo3bo4b2o12b2o$2o8bo5bo3b2o$2o8bo3bob2o4b
obo$10bo5bo7bo$11bo3bo$12b2o!
", vec![
        vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,  true, false, false, false, false, false, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,  true, false,  true, false, false, false, false, false, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false, false, false,  true,  true, false, false, false, false, false, false,  true,  true, false, false, false, false, false, false, false, false, false, false, false, false,  true,  true],
        vec![false, false, false, false, false, false, false, false, false, false, false,  true, false, false, false,  true, false, false, false, false,  true,  true, false, false, false, false, false, false, false, false, false, false, false, false,  true,  true],
        vec![ true,  true, false, false, false, false, false, false, false, false,  true, false, false, false, false, false,  true, false, false, false,  true,  true, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        vec![ true,  true, false, false, false, false, false, false, false, false,  true, false, false, false,  true, false,  true,  true, false, false, false, false,  true, false,  true, false, false, false, false, false, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false,  true, false, false, false, false, false,  true, false, false, false, false, false, false, false,  true, false, false, false, false, false, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false, false,  true, false, false, false,  true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false, false, false,  true,  true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
    ])]
    fn test_parse_file(#[case] contents: &str, #[case] expected: Grid) {
        let actual = parse_file(contents).unwrap();
        assert_eq!(expected, actual);
    }
}
