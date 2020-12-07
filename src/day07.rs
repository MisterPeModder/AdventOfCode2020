use regex::Regex;
use std::collections::HashMap;

fn has_gold<'a>(
    bag: &'a str,
    contents: &[&'a str],
    bags: &HashMap<&'a str, Vec<&'a str>>,
    bags_with_gold: &mut HashMap<&'a str, bool>,
) -> bool {
    match bags_with_gold.get(bag).copied() {
        Some(val) => val,
        None => {
            let val = contents
                .iter()
                .any(|&b| has_gold(b, bags.get(b).unwrap(), bags, bags_with_gold));

            bags_with_gold.insert(bag, val);
            val
        }
    }
}

fn count_required_bags<'a>(
    bag: &'a str,
    required: &[(&'a str, usize)],
    bags: &HashMap<&'a str, Vec<(&'a str, usize)>>,
    bags_required: &mut HashMap<&'a str, usize>,
) -> usize {
    match bags_required.get(bag).copied() {
        Some(val) => val,
        None if required.is_empty() => {
            bags_required.insert(bag, 0);
            0
        }
        None => {
            let val = required
                .iter()
                .map(|&(b, c)| {
                    c + count_required_bags(b, bags.get(b).unwrap(), bags, bags_required) * c
                })
                .sum();

            bags_required.insert(bag, val);
            val
        }
    }
}

#[aoc(day07, part1)]
pub fn day07_part1(input: &str) -> usize {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"(\w+ \w+) bag").unwrap();
    }
    let bags: HashMap<&str, Vec<&str>> = input
        .lines()
        .filter_map(|l| {
            let mut caps = PATTERN.captures_iter(l);
            let key = caps.next()?.get(1)?.as_str();
            let val: Vec<&str> = caps.filter_map(|c| c.get(1).map(|m| m.as_str())).collect();

            if val.len() == 1 && val[0] == "no other" {
                Some((key, Vec::new()))
            } else {
                Some((key, val))
            }
        })
        .collect();
    let mut bags_with_gold = HashMap::with_capacity(bags.len());

    bags_with_gold.insert("shiny gold", true);
    bags.iter()
        .filter(|&(&k, v)| has_gold(k, v, &bags, &mut bags_with_gold))
        .count()
        - 1
}

#[aoc(day07, part2)]
pub fn day07_part2(input: &str) -> usize {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"(?:(\d+) )?(\w+ \w+) bag").unwrap();
    }
    let bags: HashMap<&str, Vec<(&str, usize)>> = input
        .lines()
        .filter_map(|l| {
            let mut caps = PATTERN.captures_iter(l);
            let key = caps.next()?.get(2)?.as_str();
            let val: Vec<(&str, usize)> = caps
                .filter_map(|c| Some((c.get(2)?.as_str(), c.get(1)?.as_str().parse().ok()?)))
                .collect();

            if val.len() == 1 && val[0].0 == "no other" {
                Some((key, Vec::new()))
            } else {
                Some((key, val))
            }
        })
        .collect();
    count_required_bags(
        "shiny gold",
        bags.get("shiny gold").unwrap(),
        &bags,
        &mut HashMap::with_capacity(bags.len()),
    )
}
