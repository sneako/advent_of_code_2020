use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .trim()
        .split("\n")
        .map(|line| parse_line(&line))
        .collect()
}

fn parse_line(line: &str) -> Vec<bool> {
    let mut parsed = Vec::new();
    for c in line.chars() {
        match c {
            '#' => parsed.push(true),
            _ => parsed.push(false),
        }
    }
    parsed
}

fn next_coords(
    iteration: usize,
    down_step: usize,
    right_step: usize,
    width: usize,
) -> (usize, usize) {
    (iteration * down_step, iteration * right_step % width)
}

fn tree_count(map: &Vec<Vec<bool>>, down_step: usize, right_step: usize) -> usize {
    let mut tree_count = 0;
    let width = map[0].len();
    let height = map.len();

    for i in 1..height {
        let (down, right) = next_coords(i, down_step, right_step, width);
        if down <= height && map[down][right] {
            tree_count += 1;
        }
    }
    tree_count
}

#[aoc(day3, part1)]
pub fn part_one(input: &Vec<Vec<bool>>) -> usize {
    tree_count(&input, 1, 3)
}

#[aoc(day3, part2)]
pub fn part_two(input: &Vec<Vec<bool>>) -> usize {
    let slopes = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

    slopes.iter().fold(1, |acc, (down_step, right_step)| {
        tree_count(&input, *down_step as usize, *right_step as usize) * acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
"#;
        let parsed = parse_input(input);
        assert_eq!(part_one(&parsed), 7);
        assert_eq!(part_two(&parsed), 336);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day3.txt");
        let parsed = parse_input(&input);
        assert_eq!(part_one(&parsed), 278);
        assert_eq!(part_two(&parsed), 9709761600);
    }
}
