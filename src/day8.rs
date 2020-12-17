use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

enum Instruction {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

#[derive(Debug)]
enum ComputationResult {
    Loop(i64),
    Completed(i64),
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
    let finish = input.len();

    match tick(input, 0, &mut ran, 0, finish) {
        ComputationResult::Loop(val) => val,
        ComputationResult::Completed(val) => val,
    }
}

fn tick(
    input: &Vec<Instruction>,
    acc: i64,
    ran: &mut HashSet<usize>,
    current: usize,
    finish: usize,
) -> ComputationResult {
    if ran.contains(&current) {
        ComputationResult::Loop(acc)
    } else if current >= finish {
        ComputationResult::Completed(acc)
    } else {
        ran.insert(current);
        match &input[current] {
            Instruction::Nop(_) => exec_nop(input, acc, ran, current, finish),
            Instruction::Acc(arg) => exec_acc(input, acc, ran, current, finish, arg),
            Instruction::Jmp(arg) => exec_jmp(input, acc, ran, current, finish, arg),
        }
    }
}

#[aoc(day8, part2)]
fn part_two(input: &Vec<Instruction>) -> i64 {
    let mut ran = HashSet::<usize>::new();
    let finish = input.len();
    match repair_corrupted_instruction(input, 0, &mut ran, 0, finish) {
        ComputationResult::Completed(val) => val,
        _ => panic!("could not repair!"),
    }
}

fn repair_corrupted_instruction(
    input: &Vec<Instruction>,
    acc: i64,
    ran: &mut HashSet<usize>,
    current: usize,
    finish: usize,
) -> ComputationResult {
    if ran.contains(&current) {
        ComputationResult::Loop(acc)
    } else if current >= finish {
        ComputationResult::Completed(acc)
    } else {
        ran.insert(current);
        match &input[current] {
            Instruction::Acc(arg) => {
                repair_corrupted_instruction(input, acc + arg, ran, current + 1, finish)
            }
            Instruction::Nop(arg) => match exec_jmp(input, acc, &mut ran.clone(), current, finish, arg) {
                ComputationResult::Loop(_) => {
                    repair_corrupted_instruction(input, acc, ran, current + 1, finish)
                }
                complete => complete,
            },
            Instruction::Jmp(arg) => match exec_nop(input, acc, &mut ran.clone(), current, finish) {
                ComputationResult::Loop(_) => repair_corrupted_instruction(
                    input,
                    acc,
                    ran,
                    (current as i64 + arg) as usize,
                    finish,
                ),
                complete => complete,
            },
        }
    }
}

fn exec_nop(
    input: &Vec<Instruction>,
    acc: i64,
    ran: &mut HashSet<usize>,
    current: usize,
    finish: usize,
) -> ComputationResult {
    tick(input, acc, ran, current + 1, finish)
}

fn exec_jmp(
    input: &Vec<Instruction>,
    acc: i64,
    ran: &mut HashSet<usize>,
    current: usize,
    finish: usize,
    arg: &i64,
) -> ComputationResult {
    tick(input, acc, ran, (current as i64 + arg) as usize, finish)
}

fn exec_acc(
    input: &Vec<Instruction>,
    acc: i64,
    ran: &mut HashSet<usize>,
    current: usize,
    finish: usize,
    arg: &i64,
) -> ComputationResult {
    tick(input, acc + arg, ran, current + 1, finish)
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
        assert_eq!(part_one(&parsed), 5);
        assert_eq!(part_two(&parsed), 8);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day8.txt");
        let parsed = parse_input(input);
        assert_eq!(part_one(&parsed), 1475);
        assert_eq!(part_two(&parsed), 1270);
    }
}
