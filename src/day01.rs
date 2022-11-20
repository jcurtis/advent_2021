use crate::parse;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    let input = parse::input_to_integer_vec(input);
    input
}

#[aoc(day1, part1)]
pub fn part_1(input: &Vec<i32>) -> i32 {
    let mut count = 0;
    for (i, num) in input.iter().enumerate() {
        if i == 0 {
            continue;
        };

        let prev = input.get(i - 1).unwrap();
        if num > prev {
            count = count + 1;
        }
    }
    count
}

#[aoc(day1, part2)]
pub fn part_2(input: &Vec<i32>) -> i32 {
    let mut sums: Vec<i32> = vec![];

    for i in 0..input.len() - 2 {
        sums.push(input[i] + input[i + 1] + input[i + 2]);
    }

    part_1(&sums)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample_1() {
        let sample = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let answer = super::part_1(&sample);
        assert_eq!(answer, 7);
    }

    #[test]
    fn test_sample_2() {
        let sample = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let answer = super::part_2(&sample);
        assert_eq!(answer, 5);
    }
}
