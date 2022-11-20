pub fn run() {
    let input_file = "input/2021/input02.txt";
    let answer = part_1::solve(input_file);
    println!("part 1: {answer}");

    let answer = part_2::solve(input_file);
    println!("part 2: {answer}");
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Action {
    Forward,
    Up,
    Down,
    None,
}

mod part_1 {
    use std::fs;
    use std::path;
    use std::str;

    use super::parse_instruction;
    use super::Action;

    pub fn solve(file_name: &str) -> i32 {
        let path = path::Path::new(file_name);
        let contents = fs::read_to_string(path).expect("file not found");

        let mut hor = 0;
        let mut depth = 0;

        let instructions = contents.lines().map(parse_instruction);

        for (action, count) in instructions {
            match action {
                Action::Forward => hor = hor + count,
                Action::Up => depth = depth - count,
                Action::Down => depth = depth + count,
                _ => {}
            }
        }

        hor * depth
    }

    mod tests {
        #[test]
        fn part_1_sample() {
            let sample_file = "test_input/2021/sample02.txt";
            assert_eq!(super::solve(sample_file), 150);
        }
    }
}

mod part_2 {
    use std::fs;
    use std::path;
    use std::str;

    use super::parse_instruction;
    use super::Action;

    pub fn solve(file_name: &str) -> i32 {
        let mut hor = 0;
        let mut depth = 0;
        let mut aim = 0;

        let path = path::Path::new(file_name);
        let contents = fs::read_to_string(path).expect("file not found");
        let instructions = contents.lines().map(parse_instruction);

        for (action, count) in instructions {
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

    #[test]
    fn sample() {
        let sample_file = "test_input/2021/sample02.txt";
        assert_eq!(solve(sample_file), 900);
    }
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

mod tests {
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
}
