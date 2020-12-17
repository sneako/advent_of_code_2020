use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<u8> {
    let mut parsed: Vec<u8> = input
        .trim()
        .split("\n")
        .map(|val| val.parse::<u8>().unwrap())
        .collect();
    parsed.sort();
    parsed
}

#[aoc(day10, part1)]
pub fn part_one(input: &Vec<u8>) -> u32 {
    let mut full_input = vec![0];
    full_input.extend(input.clone());
    let device_rating = input.last().unwrap() + 3;
    full_input.push(device_rating);
    let mut differences = Vec::<u8>::new();

    for chunk in full_input.windows(2) {
        differences.push(chunk[1] - chunk[0]);
    }
    let ones = differences.iter().filter(|&val| *val == 1).count();
    let threes = differences.iter().filter(|&val| *val == 3).count();
    (ones * threes) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example1 = r#"16
10
15
5
1
11
7
19
6
12
4"#;

        let parsed1 = parse_input(example1);
        assert_eq!(part_one(&parsed1), 35);

        let example2 = r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
"#;

        let parsed2 = parse_input(example2);
        assert_eq!(part_one(&parsed2), 220);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day10.txt");
        let parsed = parse_input(input);
        assert_eq!(part_one(&parsed), 1876);
    }
}
