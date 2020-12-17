use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .split("\n")
        .map(|val| val.parse::<u64>().unwrap())
        .collect()
}

#[aoc(day9, part1)]
pub fn part_one(input: &Vec<u64>) -> u64 {
    let preamble_len = 25;
    match input[preamble_len..]
        .iter()
        .enumerate()
        .find(|(index, val)| {
            cannot_produce_from_leading(*index + preamble_len, preamble_len, **val, &input)
        }) {
        Some((_, result)) => *result,
        _ => panic!("no answer found"),
    }
}

#[aoc(day9, part2)]
pub fn part_two(input: &Vec<u64>) -> u64 {
    let target = part_one(input);

    match find_contiguous_set(input, target) {
        Some(set) => {
            let mut sorted = set.clone();
            sorted.sort();
            sorted.first().unwrap() + sorted.last().unwrap()
        }
        _ => panic!("no answer found"),
    }
}

fn find_contiguous_set(input: &Vec<u64>, target: u64) -> Option<Vec<u64>> {
    let len = input.len();

    for i in 0..len {
        let mut result = Vec::<u64>::new();
        let mut sum = 0;
        let mut idx = i;
        while sum < target {
            let current = input[idx];
            sum += current;
            idx += 1;
            result.push(current);
        }

        if sum == target {
            return Some(result);
        }
    }
    None
}

fn cannot_produce_from_leading(
    index: usize,
    preamble_len: usize,
    val: u64,
    input: &Vec<u64>,
) -> bool {
    for i in index - preamble_len..index {
        for j in index - preamble_len + 1..index {
            if i != j && val == input[i] + input[j] {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day9.txt");
        let parsed = parse_input(input);
        assert_eq!(part_one(&parsed), 32321523);
        assert_eq!(part_two(&parsed), 4794981);
    }
}
