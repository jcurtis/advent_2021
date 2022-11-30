#[aoc(day7, part1)]
fn part_1(input: &str) -> u32 {
    let input: Vec<u32> = input.split(',').map(|num| num.parse().unwrap()).collect();
    let mut min_fuel = calc_fuel_part_1(&input, 0);

    for i in 1..input.len() {
        let fuel = calc_fuel_part_1(&input, i.try_into().unwrap());
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }

    min_fuel
}

fn calc_fuel_part_1(input: &[u32], pos: u32) -> u32 {
    input.iter().map(|num| num.abs_diff(pos)).sum()
}

#[aoc(day7, part2)]
fn part_2(input: &str) -> u32 {
    let input: Vec<u32> = input.split(',').map(|num| num.parse().unwrap()).collect();
    let mut min_fuel = calc_fuel_part_2(&input, 0);

    for i in 1..input.len() {
        let fuel = calc_fuel_part_2(&input, i.try_into().unwrap());
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }

    min_fuel
}

fn calc_fuel_part_2(input: &[u32], pos: u32) -> u32 {
    let pos = pos as f32;
    input
        .iter()
        .map(|num| {
            let num = *num as f32;
            let diff = (num - pos).abs();
            ((diff / 2.0) * (1.0 + diff)) as u32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(part_1(input), 37);
    }

    #[test]
    fn test_calc_fuel_part_1() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let input: Vec<u32> = input.split(',').map(|num| num.parse().unwrap()).collect();

        assert_eq!(calc_fuel_part_1(&input, 1), 41);
        assert_eq!(calc_fuel_part_1(&input, 3), 39);
        assert_eq!(calc_fuel_part_1(&input, 10), 71);
    }

    #[test]
    fn test_part_2() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(part_2(input), 168);
    }

    #[test]
    fn test_calc_fuel_part_2() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let input: Vec<u32> = input.split(',').map(|num| num.parse().unwrap()).collect();

        assert_eq!(calc_fuel_part_2(&input, 5), 168);
        assert_eq!(calc_fuel_part_2(&input, 2), 206);
    }
}
