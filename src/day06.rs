#[aoc(day6, part1, naive)]
fn part_1(input: &str) -> usize {
    let days = 80;
    let mut state: Vec<u32> = input
        .split(",")
        .map(|digit| digit.parse().unwrap())
        .collect();

    let mut new_spawn_count = 0;
    for _ in 1..=days {
        for i in 0..state.len() {
            match state[i] {
                0 => {
                    state[i] = 6;
                    new_spawn_count += 1;
                }
                1..=8 => {
                    state[i] -= 1;
                }
                _ => println!("Fish out of water {i}: {}", &state[i]),
            };
        }

        for _ in 0..new_spawn_count {
            state.push(8);
        }

        new_spawn_count = 0;
    }

    state.len()
}

#[aoc(day6, part1, better)]
fn part_1_v2(input: &str) -> u64 {
    solve(input, 80)
}

#[aoc(day6, part2)]
fn part_2(input: &str) -> u64 {
    solve(input, 256)
}

fn solve(input: &str, days: usize) -> u64 {
    let mut state = vec![0; 9];

    input.split(",").for_each(|fish| {
        let fish = fish.parse::<usize>().unwrap();
        state[fish] += 1;
    });

    for _ in 1..=days {
        let zeroes = state[0];
        state.rotate_left(1);
        state[6] = state[6] + zeroes;
    }

    state.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::parse::read_file;

    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_file("test_input/input06.txt");
        let answer = part_1(&input);
        assert_eq!(answer, 5934);
    }

    #[test]
    fn test_part_2() {
        let input = read_file("test_input/input06.txt");
        let answer = part_2(&input);
        assert_eq!(answer, 26984457539);
    }
}
