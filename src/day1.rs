use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<u32> {
    input
        .trim()
        .split("\n")
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn part_one(input: &Vec<u32>) -> u32 {
    let len = input.len();
    for i in 0..len {
        let target = 2020 - input[i];
        for j in (i + 1)..len {
            if input[j] == target {
                return input[i] * input[j];
            }
        }
    }

    panic!("No answer found");
}

#[aoc(day1, part2)]
pub fn part_two(input: &Vec<u32>) -> u32 {
    let len = input.len();
    for i in 0..len {
        for j in (i + 1)..len {
            for k in (j + 1)..len {
                if input[i] + input[j] + input[k] == 2020 {
                    return input[i] * input[j] * input[k];
                }
            }
        }
    }

    panic!("No answer found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"1721
979
366
299
675
1456"#;
        let parsed = parse_input(input);
        assert_eq!(part_one(&parsed), 514579);
        assert_eq!(part_two(&parsed), 241861950);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day1.txt");
        let parsed = parse_input(input);
        assert_eq!(part_one(&parsed), 980499);
        assert_eq!(part_two(&parsed), 200637446);
    }
}
