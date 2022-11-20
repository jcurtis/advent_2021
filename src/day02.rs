#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Action {
    Forward,
    Up,
    Down,
    None,
}

fn parse_instruction(instruction: &str) -> (Action, i32) {
    let mut split = instruction.split_whitespace();
    let action = match split.next().unwrap() {
        "forward" => Action::Forward,
        "up" => Action::Up,
        "down" => Action::Down,
        _ => Action::None,
    };
    let count: i32 = split.next().unwrap().parse().unwrap();
    (action, count)
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(Action, i32)> {
    let instructions = input.lines().map(parse_instruction);
    Vec::from_iter(instructions)
}

#[aoc(day2, part1)]
pub fn part_1(input: &Vec<(Action, i32)>) -> i32 {
    let mut hor = 0;
    let mut depth = 0;

    for (action, count) in input {
        match action {
            Action::Forward => hor = hor + count,
            Action::Up => depth = depth - count,
            Action::Down => depth = depth + count,
            _ => {}
        }
    }

    hor * depth
}

#[aoc(day2, part2)]
pub fn part_2(input: &Vec<(Action, i32)>) -> i32 {
    let mut hor = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (action, count) in input {
        match action {
            Action::Forward => {
                hor = hor + count;
                depth = depth + (aim * count);
            }
            Action::Up => {
                aim = aim - count;
            }
            Action::Down => {
                aim = aim + count;
            }
            _ => {}
        }
    }

    hor * depth
}

#[cfg(test)]
mod tests {
    use crate::parse::read_file;

    #[test]
    fn parse_instruction() {
        let (action, count) = super::parse_instruction("forward 5");
        assert_eq!(action, super::Action::Forward);
        assert_eq!(count, 5);

        let (action, count) = super::parse_instruction("up 22");
        assert_eq!(action, super::Action::Up);
        assert_eq!(count, 22);

        let (action, count) = super::parse_instruction("down -1");
        assert_eq!(action, super::Action::Down);
        assert_eq!(count, -1);
    }

    #[test]
    fn part_1_sample() {
        let input = read_file("test_input/2021/sample02.txt");
        let input = super::input_generator(&input);
        assert_eq!(super::part_1(&input), 150);
    }

    #[test]
    fn part_2_sample() {
        let input = read_file("test_input/2021/sample02.txt");
        let input = super::input_generator(&input);
        assert_eq!(super::part_2(&input), 900);
    }
}
