use std::collections::HashMap;

use itertools::Itertools;

use crate::parse::parse_i32;

type Coord = (i32, i32);

struct Line {
    from: Coord,
    to: Coord,
}

type Input = Vec<Line>;

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Input {
    input
        .trim()
        .lines()
        .into_iter()
        .map(|line| {
            let line = line.split_whitespace().collect::<Vec<&str>>();

            let from = line[0];
            let from = from.split(",").map(parse_i32).collect::<Vec<i32>>();

            let to = line[2];
            let to = to.split(",").map(parse_i32).collect::<Vec<i32>>();

            Line {
                from: (from[0], from[1]),
                to: (to[0], to[1]),
            }
        })
        .collect()
}

#[aoc(day5, part1)]
fn part_1(input: &Input) -> usize {
    let mut grid = HashMap::new();
    for line in input {
        if is_perpendicular(&line) {
            let range = line_range(&line);
            for coord in range {
                let val: i32 = match grid.get(&coord) {
                    Some(i) => i + 1,
                    None => 1,
                };
                grid.insert(coord, val);
            }
        }
    }

    grid.into_iter()
        .filter(|(_, count)| *count >= 2)
        .collect_vec()
        .len()
}

fn line_range(input: &Line) -> Vec<Coord> {
    let mut res = vec![];

    if input.from.0 == input.to.0 {
        let range = if input.from.1 <= input.to.1 {
            (input.from.1..=input.to.1).collect_vec()
        } else {
            (input.to.1..=input.from.1).rev().collect_vec()
        };

        for i in range {
            res.push((input.from.0, i));
        }
    } else if input.from.1 == input.to.1 {
        let range = if input.from.0 <= input.to.0 {
            (input.from.0..=input.to.0).collect_vec()
        } else {
            (input.to.0..=input.from.0).rev().collect_vec()
        };

        for i in range {
            res.push((i, input.from.1));
        }
    }

    res
}

fn is_perpendicular(input: &Line) -> bool {
    input.from.0 == input.to.0 || input.from.1 == input.to.1
}

#[cfg(test)]
mod tests {
    use crate::parse::read_file;

    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_file("test_input/input05.txt");
        let input = input_generator(&input);
        assert_eq!(part_1(&input), 5);
    }

    #[test]
    fn test_line_range() {
        let line = Line {
            from: (0, 0),
            to: (0, 2),
        };
        let range = line_range(&line);
        assert_eq!(range, vec![(0, 0), (0, 1), (0, 2)]);

        let line = Line {
            from: (0, 2),
            to: (0, 0),
        };
        let range = line_range(&line);
        assert_eq!(range, vec![(0, 2), (0, 1), (0, 0)]);

        let line = Line {
            from: (3, 2),
            to: (5, 2),
        };
        let range = line_range(&line);
        assert_eq!(range, vec![(3, 2), (4, 2), (5, 2)]);

        let line = Line {
            from: (5, 2),
            to: (3, 2),
        };
        let range = line_range(&line);
        assert_eq!(range, vec![(5, 2), (4, 2), (3, 2)]);
    }

    #[test]
    fn test_is_perpendicular() {
        let line = Line {
            from: (0, 8),
            to: (0, 1000),
        };
        assert!(is_perpendicular(&line));

        let line = Line {
            from: (5, 8),
            to: (1, 8),
        };
        assert!(is_perpendicular(&line));

        let line = Line {
            from: (5, 8),
            to: (1, 9),
        };
        assert!(!is_perpendicular(&line));
    }
}
