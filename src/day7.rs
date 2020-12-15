use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

struct Bag {
    description: String,
    children: Vec<(u32, String)>,
}

#[aoc_generator(day7)]
fn parse_input(input: &str) -> HashMap<String, Bag> {
    let mut dict = HashMap::<String, Bag>::new();
    for row in input.trim().split("\n") {
        let bag = parse_new_row(row);
        dict.insert(bag.description.clone(), bag);
    }
    dict
}

fn parse_new_row(row: &str) -> Bag {
    let split: Vec<&str> = row.split("bags contain").collect();
    let mut children = Vec::<(u32, String)>::new();
    for (count, child) in parse_children(split[1]) {
        children.push((count, child));
    }
    Bag {
        description: String::from(split[0].trim()),
        children: children,
    }
}

fn parse_children(input: &str) -> Vec<(u32, String)> {
    let mut output = Vec::new();
    for child in input.split(", ") {
        match child {
            " no other bags." => (),
            _ => output.push(parse_child(child)),
        }
    }
    output
}

fn parse_child(child: &str) -> (u32, String) {
    let parsed: Vec<&str> = child.trim().split(" ").collect();
    let mut name = String::new();
    name.push_str(parsed[1]);
    name.push_str(" ");
    name.push_str(parsed[2]);
    (parsed[0].parse::<u32>().unwrap(), name)
}

#[aoc(day7, part1)]
fn part_one(input: &HashMap<String, Bag>) -> u32 {
    let mut count = 0;
    for (_, bag) in input {
        if leads_to_shiny_gold(bag, input) {
            count += 1;
        }
    }
    count
}

fn leads_to_shiny_gold(bag: &Bag, dict: &HashMap<String, Bag>) -> bool {
    if bag.children.iter().any(|(_, child)| child == "shiny gold") {
        true
    } else {
        bag.children.iter().any(|(_, child)| match dict.get(child) {
            Some(bag) => leads_to_shiny_gold(bag, dict),
            _ => false,
        })
    }
}

#[aoc(day7, part2)]
fn part_two(input: &HashMap<String, Bag>) -> u32 {
    match input.get("shiny gold") {
        Some(shiny_gold) => sum_child_count(&shiny_gold, input) - 1,
        _ => 0,
    }
}

fn sum_child_count(bag: &Bag, dict: &HashMap<String, Bag>) -> u32 {
    bag.children
        .iter()
        .fold(1, |acc, (count, child)| match dict.get(child) {
            Some(child_bag) => acc + (count * sum_child_count(child_bag, dict)),
            _ => acc,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;
        let parsed = parse_input(input);
        assert_eq!(part_one(&parsed), 4);
        assert_eq!(part_two(&parsed), 32);

        let second = r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."#;
        let second_parsed = parse_input(second);
        assert_eq!(part_two(&second_parsed), 126);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day7.txt");
        let parsed = parse_input(input);
        assert_eq!(part_one(&parsed), 172);
        assert_eq!(part_two(&parsed), 39645);
    }
}
