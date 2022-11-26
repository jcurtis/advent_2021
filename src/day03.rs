pub struct ParsedInput {
    num_chars: usize,
    counts: Vec<(i32, i32)>,
}

pub fn count_all_bit_pos(input: &str) -> ParsedInput {
    let num_chars = input.lines().next().unwrap().len();
    let mut counts = vec![(0, 0); num_chars];
    for line in input.trim().lines() {
        for (i, c) in line.chars().enumerate() {
            match c {
                '0' => counts[i].0 += 1,
                '1' => counts[i].1 += 1,
                _ => {}
            }
        }
    }

    ParsedInput { num_chars, counts }
}

#[aoc(day3, part1)]
pub fn part_1(input: &str) -> i32 {
    let input = count_all_bit_pos(input);
    let mut gamma = vec![0; input.num_chars];
    for (i, count) in input.counts.iter().enumerate() {
        if count.0 < count.1 {
            gamma[i] = 1;
        }
    }
    let epsilon = flip_bits(&gamma);

    let gamma_dec = bits_to_int(&gamma);
    let epsilon_dec = bits_to_int(&epsilon);

    gamma_dec * epsilon_dec
}

#[aoc(day3, part2)]
pub fn part_2(input: &str) -> i32 {
    let input = input.lines().collect::<Vec<&str>>();
    let num_chars = input[0].len();

    let mut oxygen: Option<&str> = None;
    let mut filtered = input.clone();
    for pos in 0..num_chars {
        let input = filtered.clone();
        let counted = count_bit_pos(&input, pos);
        let by = if counted.0 <= counted.1 { '1' } else { '0' };

        filtered = input
            .into_iter()
            .filter(|line| line.chars().nth(pos).unwrap() == by)
            .collect();

        if filtered.len() <= 1 {
            oxygen = Some(filtered[0]);
            break;
        }
    }

    let mut co2: Option<&str> = None;
    let mut filtered = input.clone();
    for pos in 0..num_chars {
        let input = filtered.clone();
        let counted = count_bit_pos(&input, pos);
        let by = if counted.0 <= counted.1 { '0' } else { '1' };

        filtered = input
            .into_iter()
            .filter(|line| line.chars().nth(pos).unwrap() == by)
            .collect();

        if filtered.len() <= 1 {
            co2 = Some(filtered[0]);
            break;
        }
    }

    let oxygen = i32::from_str_radix(oxygen.unwrap(), 2).unwrap();
    let co2 = i32::from_str_radix(co2.unwrap(), 2).unwrap();

    oxygen * co2
}

fn count_bit_pos(input: &[&str], pos: usize) -> (i32, i32) {
    let mut count = (0, 0);
    for item in input {
        let c = item.chars().nth(pos).unwrap();
        match c {
            '0' => count.0 += 1,
            '1' => count.1 += 1,
            _ => {}
        }
    }
    count
}

pub fn filter_by_bit_pos<'a>(
    input: impl Iterator<Item = &'a str>,
    pos: usize,
    by: char,
) -> Vec<&'a str> {
    input
        .filter(|line| line.chars().nth(pos).unwrap() == by)
        .collect()
}

fn flip_bits(input: &[i32]) -> Vec<i32> {
    input
        .iter()
        .map(|n| match n {
            0 => 1,
            1 => 0,
            _ => 0,
        })
        .collect()
}

fn bits_to_int(input: &[i32]) -> i32 {
    let string: String = input.iter().map(|n| n.to_string()).collect();
    i32::from_str_radix(&string, 2).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn part_1_sample() {
        let input = fs::read_to_string("test_input/input03.txt").unwrap();
        // let input = super::input_generator(&input);
        assert_eq!(super::part_1(&input), 198);
    }

    #[test]
    fn part_2_sample() {
        let input = fs::read_to_string("test_input/input03.txt").unwrap();
        // let input = super::input_generator(&input);
        assert_eq!(super::part_2(&input), 230);
    }

    #[test]
    fn flip_bits() {
        assert_eq!(super::flip_bits(&vec![1, 0, 1, 1, 0]), vec![0, 1, 0, 0, 1]);
    }

    #[test]
    fn bits_to_int() {
        assert_eq!(super::bits_to_int(&vec![1, 0, 1, 1, 0]), 22);
    }
}
