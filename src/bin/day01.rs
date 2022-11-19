fn main() {
    let input = advent::parse::parse_file_integers("input/2021/day01.txt");

    let answer1 = part_1::solve(&input);
    println!("Solve for day 1 part 1: {answer1}");

    let answer2 = part_2::solve(&input);
    println!("Solve for day 1 part 2: {answer2}")
}

mod part_1 {
    pub fn solve(input: &Vec<i32>) -> i32 {
        let mut count = 0;
        for (i, num) in input.iter().enumerate() {
            // println!("is {i} equal to 0");
            if i == 0 {
                continue;
            };

            let prev = input.get(i - 1).unwrap();
            // println!("Comparing {num} to {prev}");
            if num > prev {
                // println!("{num} is larger than {prev}");
                count = count + 1;
            }
        }
        count
    }

    mod tests {
        #[test]
        fn test_sample() {
            let sample = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
            let answer = super::solve(&sample);
            assert_eq!(answer, 7);
        }
    }
}

mod part_2 {
    pub fn solve(input: &Vec<i32>) -> i32 {
        let mut sums: Vec<i32> = vec![];

        for i in 0..input.len() - 2 {
            sums.push(input[i] + input[i + 1] + input[i + 2]);
        }

        // println!("sums {:?}", sums);
        super::part_1::solve(&sums)
    }

    mod tests {
        #[test]
        fn test_sample() {
            let sample = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
            let answer = super::solve(&sample);
            assert_eq!(answer, 5);
        }
    }
}
