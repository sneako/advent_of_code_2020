use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[derive(Debug)]
enum Instruction {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .split("\n")
        .map(|line| match line.split(" ").collect::<Vec<&str>>()[..] {
            ["nop", arg] => Instruction::Nop(arg.parse::<i64>().unwrap()),
            ["acc", arg] => Instruction::Acc(arg.parse::<i64>().unwrap()),
            ["jmp", arg] => Instruction::Jmp(arg.parse::<i64>().unwrap()),
            _ => panic!("invalid input"),
        })
        .collect()
}

#[aoc(day8, part1)]
fn part_one(input: &Vec<Instruction>) -> i64 {
    let mut ran = HashSet::<usize>::new();
    tick(input, 0, &mut ran, 0)
}

fn tick(input: &Vec<Instruction>, acc: i64, ran: &mut HashSet<usize>, current: usize) -> i64 {
    if ran.contains(&current) {
        acc
    } else {
        ran.insert(current);
        match &input[current] {
            Instruction::Nop(_) => tick(input, acc, ran, current + 1),
            Instruction::Acc(arg) => tick(input, acc + arg, ran, current + 1),
            Instruction::Jmp(arg) => tick(input, acc, ran, (current as i64 + arg) as usize),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;
        let parsed = parse_input(input);
        println!("{:?}", parsed);
        assert_eq!(part_one(&parsed), 5);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day8.txt");
        let parsed = parse_input(input);
        assert_eq!(part_one(&parsed), 1475);
    }
}
