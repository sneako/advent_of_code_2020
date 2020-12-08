use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone)]
struct Seat {
    id: u16,
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<Seat> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let row = calc_row(&line[..7]);
            let col = calc_col(&line[7..]);
            Seat { id: row * 8 + col }
        })
        .collect()
}

#[aoc(day5, part1)]
fn part_one(input: &Vec<Seat>) -> u16 {
    input.iter().max_by_key(|seat| seat.id).unwrap().id
}

#[aoc(day5, part2)]
fn part_two(input: &Vec<Seat>) -> u16 {
    let mut clone = input.to_vec();
    clone.sort_by(|a, b| a.id.cmp(&b.id));
    let mut my_seat = None;

    for i in 0..clone.len() - 1 {
        if clone[i + 1].id - clone[i].id == 2 {
            my_seat = Some(clone[i].id + 1);
            break;
        }
    }
    my_seat.unwrap()
}

fn calc_row(input: &str) -> u16 {
    let mut min = 0;
    let mut max = 127;
    let mut last_char = None;
    let chars = input.chars();

    for c in chars {
        last_char = Some(c);
        let next = min + (max - min) / 2;
        match c {
            'F' => max = next,
            'B' => min = next + 1,
            _ => (),
        }
    }

    match last_char {
        Some('F') => max,
        _ => min,
    }
}

fn calc_col(input: &str) -> u16 {
    let mut min = 0;
    let mut max = 7;
    let mut last_char = None;
    let chars = input.chars();

    for c in chars {
        last_char = Some(c);
        let next = min + (max - min) / 2;
        match c {
            'L' => max = next,
            'R' => min = next + 1,
            _ => (),
        }
    }

    match last_char {
        Some('L') => max,
        _ => min,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let one = parse_input("FBFBBFFRLR");
        assert_eq!(part_one(&one), 357);

        let two = parse_input("BFFFBBFRRR");
        assert_eq!(part_one(&two), 567);

        let three = parse_input("FFFBBBFRRR");
        assert_eq!(part_one(&three), 119);

        let four = parse_input("BBFFBBFRLL");
        assert_eq!(part_one(&four), 820);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day5.txt");
        let parsed = parse_input(input);
        assert_eq!(part_one(&parsed), 866);
        assert_eq!(part_two(&parsed), 583);
    }
}
