#[aoc_generator(day19)]
pub fn day19_gen(input: &str) -> (Vec<Rule>, String) {
    let (rules, messages) = input.split_once("\n\n").unwrap();
    let mut r = Vec::with_capacity(rules.lines().count());

    'outer: for rule in rules.lines() {
        let (index, rule) = rule.split_once(':').unwrap();
        let index = index.parse().unwrap();
        let mut part2 = Vec::with_capacity(2);
        let mut part1 = None;

        if r.len() <= index {
            r.resize(index + 1, Rule::Character('?'));
        }
        for part in rule.split_whitespace() {
            if part == "|" {
                part1 = Some(part2.clone());
                part2.clear();
            } else if part.starts_with('"') {
                r[index] = Rule::Character(part.chars().nth(1).unwrap());
                continue 'outer;
            } else {
                part2.push(part.parse().unwrap());
            }
        }
        r[index] = if let Some(part1) = part1 {
            Rule::Either(part1, part2)
        } else {
            Rule::List(part2)
        }
    }
    (r, messages.to_owned())
}

#[derive(Debug, Clone)]
pub enum Rule {
    Character(char),
    List(Vec<usize>),
    Either(Vec<usize>, Vec<usize>),
}

fn matches_list<'a>(mut msg: &'a str, list: &[usize], rules: &[Rule]) -> Option<&'a str> {
    for &rule in list {
        match matches_rule(msg, &rules[rule], rules) {
            Some(m) => msg = m,
            None => return None,
        }
    }
    Some(msg)
}

fn matches_rule<'a>(msg: &'a str, rule: &Rule, rules: &[Rule]) -> Option<&'a str> {
    match rule {
        Rule::Character(c) => {
            let mut chars = msg.char_indices();

            match chars.next() {
                Some((_, m)) if m == *c => match chars.next() {
                    Some((i, _)) => Some(&msg[i..]),
                    None => Some(&msg[msg.len()..]),
                },
                _ => None,
            }
        }
        Rule::List(l) => matches_list(msg, &l, rules),
        Rule::Either(l1, l2) => matches_list(msg, &l1, rules).or_else(|| matches_list(msg, &l2, rules)),
    }
}

fn matches_rules(msg: &str, rules: &[Rule]) -> bool {
    match matches_rule(msg, &rules[0], rules) {
        Some(msg) => msg.is_empty(),
        None => false
    }
}

#[aoc(day19, part1)]
pub fn day19_part1(input: &(Vec<Rule>, String)) -> usize {
    input.1.lines().filter(|&l| matches_rules(l, &input.0)).count()
}

#[aoc(day19, part2)]
pub fn day19_part2(input: &(Vec<Rule>, String)) -> usize {
    let mut new_rules = input.0.clone();

    new_rules[8] = Rule::Either(vec![42], vec![42, 8]);
    new_rules[11] = Rule::Either(vec![42, 31], vec![42, 11, 31]);
    input.1.lines().filter(|&l| matches_rules(l, &new_rules)).count()
}