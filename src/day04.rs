use regex::Regex;
use std::{collections::HashMap, convert::Infallible, str::FromStr};

pub struct Passport(HashMap<String, String>);

impl Passport {
    fn has_required_keys(&self) -> bool {
        const KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

        KEYS.iter().all(|&k| self.0.contains_key(k))
    }

    fn is_valid(&self) -> bool {
        lazy_static! {
            static ref PATTERNS: Vec<(&'static str, Regex)> = {
                let key_patterns = &[
                    ("byr", r"^19[2-9]\d|200[0-2]$"),
                    ("iyr", r"^201\d|2020$"),
                    ("eyr", r"^202\d|2030$"),
                    ("hgt", r"^1(?:[5-8]\d|9[0-3])cm|(?:59|6\d|7[0-6])in$"),
                    ("hcl", r"^#[[:xdigit:]]{6}$"),
                    ("ecl", r"^amb|blu|brn|gry|grn|hzl|oth$"),
                    ("pid", r"^\d{9}$"),
                ];
                key_patterns
                    .iter()
                    .map(|&(k, p)| (k, Regex::new(p).unwrap()))
                    .collect()
            };
        }

        PATTERNS
            .iter()
            .all(|(k, p): &(&str, Regex)| self.0.get(k as &str).map_or(false, |v| p.is_match(v)))
    }
}

impl FromStr for Passport {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Passport(
            s.split(|c| c == ' ' || c == '\n')
                .filter_map(|field| {
                    let mut parts = field.split(':');
                    let key = parts.next()?.to_owned();
                    let value = parts.next()?.to_owned();

                    match parts.next() {
                        None => Some((key, value)),
                        Some(_) => None,
                    }
                })
                .collect(),
        ))
    }
}

#[aoc_generator(day04)]
pub fn day04_gen(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .filter(|&line| !line.is_empty())
        .map(|line| Passport::from_str(line).unwrap())
        .collect()
}

#[aoc(day04, part1)]
pub fn day04_part1(input: &[Passport]) -> usize {
    input.iter().filter(|&p| p.has_required_keys()).count()
}

#[aoc(day04, part2)]
pub fn day04_part2(input: &[Passport]) -> usize {
    input.iter().filter(|&p| p.is_valid()).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        const PASSPORTS: [&str; 4] = [
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f",
            "eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
            "hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022",
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        ];

        assert!(PASSPORTS
            .iter()
            .all(|&s| Passport::from_str(s).unwrap().is_valid()));
    }
}
