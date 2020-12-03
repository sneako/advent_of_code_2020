use aoc_runner_derive::{aoc, aoc_generator};

pub struct Policy {
    subject: String,
    min: usize,
    max: usize,
}

pub struct PasswordWithPolicy {
    policy: Policy,
    password: String,
}

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<PasswordWithPolicy> {
    input
        .trim()
        .split("\n")
        .map(|line| line_to_password_with_policy(line))
        .collect()
}

fn line_to_password_with_policy(line: &str) -> PasswordWithPolicy {
    let split: Vec<&str> = line.split(": ").collect();
    let policy_input: Vec<&str> = split[0].split(" ").collect();
    let policy_min_max: Vec<&str> = policy_input[0].split("-").collect();

    PasswordWithPolicy {
        password: String::from(split[1]),
        policy: Policy {
            subject: String::from(policy_input[1]),
            min: policy_min_max[0].parse::<usize>().unwrap(),
            max: policy_min_max[1].parse::<usize>().unwrap(),
        },
    }
}

fn valid_count(
    input: &Vec<PasswordWithPolicy>,
    validate_fun: &dyn Fn(&PasswordWithPolicy) -> bool,
) -> usize {
    input
        .into_iter()
        .filter(|pw| validate_fun(&pw))
        .count()
}

fn is_valid_password_part_one(input: &PasswordWithPolicy) -> bool {
    let count = input.password.matches(&input.policy.subject).count();
    count >= input.policy.min && count <= input.policy.max
}

#[aoc(day2, part1)]
pub fn part_one(input: &Vec<PasswordWithPolicy>) -> usize {
    valid_count(input, &is_valid_password_part_one)
}

fn is_valid_password_part_two(input: &PasswordWithPolicy) -> bool {
    let chars: Vec<char> = input.password.chars().collect();
    let subject = input.policy.subject.chars().nth(0).unwrap();
    let min_char = chars[input.policy.min - 1];
    let max_char = chars[input.policy.max - 1];
    (min_char == subject && max_char != subject) || (min_char != subject && max_char == subject)
}

#[aoc(day2, part2)]
pub fn part_two(input: &Vec<PasswordWithPolicy>) -> usize {
    valid_count(input, &is_valid_password_part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
"#;
        let parsed = parse_input(input);
        assert_eq!(part_one(&parsed), 2);
        assert_eq!(part_two(&parsed), 1);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day2.txt");
        let parsed = parse_input(input);
        assert_eq!(part_one(&parsed), 636);
        assert_eq!(part_two(&parsed), 588);
    }
}
