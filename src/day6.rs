use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<String> {
    input
        .trim()
        .split("\n\n")
        .map(|group| group.to_string())
        .collect()
}

fn parse_group_part_one(group: &String) -> HashSet<char> {
    let mut parsed = HashSet::new();
    for question in group.chars() {
        match question {
            '\n' => false,
            question => parsed.insert(question),
        };
    }
    parsed
}

fn parse_group_part_two(group: &String) -> HashSet<char> {
    let individuals: Vec<HashSet<char>> = group
        .split("\n")
        .into_iter()
        .map(|individual| {
            let mut parsed = HashSet::new();
            for question in individual.chars() {
                parsed.insert(question);
            }
            parsed
        })
        .collect();
    individuals
        .iter()
        .skip(1)
        .fold(individuals[0].clone(), |acc, hs| {
            acc.intersection(hs).cloned().collect()
        })
}

#[aoc(day6, part1)]
fn part_one(input: &Vec<String>) -> u32 {
    input
        .iter()
        .map(|group| parse_group_part_one(group))
        .fold(0, |count, group| count + group.len() as u32)
}

#[aoc(day6, part2)]
fn part_two(input: &Vec<String>) -> u32 {
    input
        .iter()
        .map(|group| parse_group_part_two(group))
        .fold(0, |count, group| count + group.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;
        let parsed = parse_input(input);
        assert_eq!(part_one(&parsed), 11);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day6.txt");
        let parsed = parse_input(input);
        assert_eq!(part_one(&parsed), 6457);
        assert_eq!(part_two(&parsed), 3260);
    }
}
