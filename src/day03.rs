#[aoc(day3, part1)]
pub fn part_1(input: &str) -> i32 {
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

    let mut gamma = vec![0; num_chars];
    for (i, count) in counts.into_iter().enumerate() {
        if count.0 < count.1 {
            gamma[i] = 1;
        }
    }
    let epsilon = flip_bits(&gamma);

    let gamma_dec = bits_to_int(&gamma);
    let epsilon_dec = bits_to_int(&epsilon);

    gamma_dec * epsilon_dec
}

// #[aoc(day3, part2)]
// pub fn part_2((input, counts, num_chars): &(&str, Vec<(i32, i32)>, usize)) -> i32 {
//     0
// }

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

    // #[test]
    // fn part_2_sample() {
    //     let input = fs::read_to_string("test_input/input03.txt").unwrap();
    //     // let input = super::input_generator(&input);
    //     assert_eq!(super::part_2(&input), 230);
    // }

    #[test]
    fn flip_bits() {
        assert_eq!(super::flip_bits(&vec![1, 0, 1, 1, 0]), vec![0, 1, 0, 0, 1]);
    }

    #[test]
    fn bits_to_int() {
        assert_eq!(super::bits_to_int(&vec![1, 0, 1, 1, 0]), 22);
    }
}
