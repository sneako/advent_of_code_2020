use aoc_runner_derive::{aoc, aoc_generator};
use regex::{Captures, Regex};

struct Passport {
    id: Option<String>,
    country_id: Option<String>,
    birth_year: Option<u16>,
    issue_year: Option<u16>,
    expiration_year: Option<u16>,
    height: Option<Height>,
    eye_color: Option<EyeColor>,
    hair_color: Option<String>,
}

enum Height {
    Inches(u8),
    Centimeters(u8),
}

enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            id: None,
            country_id: None,
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            eye_color: None,
            hair_color: None,
        }
    }

    fn is_valid(passport: &Self) -> bool {
        passport.id.is_some()
            && passport.birth_year.is_some()
            && passport.issue_year.is_some()
            && passport.expiration_year.is_some()
            && passport.height.is_some()
            && passport.eye_color.is_some()
            && passport.hair_color.is_some()
    }
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<Passport> {
    input
        .trim()
        .split("\n\n")
        .map(|line| parse_line(line))
        .collect()
}

fn parse_line(line: &str) -> Passport {
    let mut passport = Passport::new();
    let split: Vec<&str> = line.split_whitespace().collect();
    for kv in split {
        let key_value: Vec<&str> = kv.split(":").collect();
        let value = key_value[1];
        match key_value[0] {
            "pid" => passport.id = parse_passport_id(value),
            "cid" => passport.country_id = Some(String::from(value)),
            "byr" => passport.birth_year = parse_year(value, 1920, 2002),
            "iyr" => passport.issue_year = parse_year(value, 2010, 2020),
            "eyr" => passport.expiration_year = parse_year(value, 2020, 2030),
            "hgt" => passport.height = parse_height(value),
            "ecl" => passport.eye_color = parse_eye_color(value),
            "hcl" => passport.hair_color = parse_hair_color(value),
            _ => (),
        }
    }

    passport
}

fn parse_passport_id(input: &str) -> Option<String> {
    let re = Regex::new("^\\d{9}$").unwrap();
    if re.is_match(input) {
        Some(String::from(input))
    } else {
        None
    }
}

fn parse_year(input: &str, min: u16, max: u16) -> Option<u16> {
    match input.parse::<u16>() {
        Ok(parsed) => {
            if parsed >= min && parsed <= max {
                Some(parsed)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn parse_eye_color(input: &str) -> Option<EyeColor> {
    match input {
        "amb" => Some(EyeColor::Amber),
        "blu" => Some(EyeColor::Blue),
        "brn" => Some(EyeColor::Brown),
        "gry" => Some(EyeColor::Gray),
        "grn" => Some(EyeColor::Green),
        "hzl" => Some(EyeColor::Hazel),
        "oth" => Some(EyeColor::Other),
        _ => None,
    }
}

fn parse_hair_color(input: &str) -> Option<String> {
    let re = Regex::new("^#[0-9a-f]{6}").unwrap();
    if re.is_match(input) {
        Some(String::from(input))
    } else {
        None
    }
}

fn parse_height(input: &str) -> Option<Height> {
    let re = Regex::new("^(\\d+)(in|cm)$").unwrap();
    match re.captures(input) {
        Some(caps) => extract_height(caps),
        _ => None,
    }
}

fn extract_height(captures: Captures) -> Option<Height> {
    captures.get(2).and_then(|unit| match unit.as_str() {
        "in" => captures.get(1).and_then(|val| parse_inches(val.as_str())),
        "cm" => captures.get(1).and_then(|val| parse_cm(val.as_str())),
        _ => None,
    })
}

fn parse_inches(input: &str) -> Option<Height> {
    match input.parse::<u8>() {
        Ok(parsed) => {
            if parsed >= 59 && parsed <= 76 {
                Some(Height::Inches(parsed))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn parse_cm(input: &str) -> Option<Height> {
    match input.parse::<u8>() {
        Ok(parsed) => {
            if parsed >= 150 && parsed <= 193 {
                Some(Height::Centimeters(parsed))
            } else {
                None
            }
        }
        _ => None,
    }
}

#[aoc(day4, part2)]
fn part_two(input: &Vec<Passport>) -> u32 {
    input
        .into_iter()
        .filter(|passport| Passport::is_valid(&passport))
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;
        let parsed = parse_input(input);
        assert_eq!(part_two(&parsed), 2);

        let invalid = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"#;
        let parsed = parse_input(invalid);
        assert_eq!(part_two(&parsed), 0);

        let valid = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#;
        let parsed = parse_input(valid);
        assert_eq!(part_two(&parsed), 4);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../input/2020/day4.txt");
        let parsed = parse_input(input);
        assert_eq!(part_two(&parsed), 156);
    }
}
