use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    iter,
    ops::RangeInclusive,
};

pub struct Tickets {
    fields: HashMap<String, (RangeInclusive<u64>, RangeInclusive<u64>)>,
    your_ticket: Vec<u64>,
    nearby_tickets: Vec<Vec<u64>>,
}

fn parse_ticket(line: &str) -> Vec<u64> {
    line.split(',').filter_map(|n| n.parse().ok()).collect()
}

#[aoc_generator(day16)]
pub fn day16_gen(input: &str) -> Tickets {
    lazy_static! {
        static ref FIELD_PATTERN: Regex =
            Regex::new(r"^(.*): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    }

    let mut parts = input.split("\n\n");
    let fields = parts
        .next()
        .unwrap()
        .lines()
        .filter_map(|l| {
            let caps = FIELD_PATTERN.captures(l)?;

            Some((
                caps[1].to_owned(),
                (
                    caps[2].parse().ok()?..=caps[3].parse().ok()?,
                    caps[4].parse().ok()?..=caps[5].parse().ok()?,
                ),
            ))
        })
        .collect();
    let your_ticket = parse_ticket(parts.next().unwrap().lines().nth(1).unwrap());
    let nearby_tickets = parts
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(parse_ticket)
        .collect();

    Tickets {
        fields,
        your_ticket,
        nearby_tickets,
    }
}

fn is_valid_field(tickets: &Tickets, ticket: u64) -> bool {
    tickets
        .fields
        .values()
        .any(|(r1, r2)| r1.contains(&ticket) || r2.contains(&ticket))
}

#[aoc(day16, part1)]
pub fn day16_part1(input: &Tickets) -> u64 {
    input
        .nearby_tickets
        .iter()
        .filter_map(|ticket| ticket.iter().copied().find(|&f| !is_valid_field(input, f)))
        .sum()
}

fn get_field_candidates<'a>(tickets: &'a Tickets) -> Vec<HashSet<&'a str>> {
    let field_count = tickets.your_ticket.len();
    let mut field_candidates: Vec<HashSet<&str>> =
        vec![tickets.fields.keys().map(|k| &k as &str).collect(); field_count];
    let valid = iter::once(&tickets.your_ticket).chain(
        tickets
            .nearby_tickets
            .iter()
            .filter(|&ticket| ticket.iter().all(|&f| is_valid_field(tickets, f))),
    );

    for ticket in valid {
        for (i, &field) in ticket.iter().enumerate() {
            for (name, (r1, r2)) in tickets.fields.iter() {
                if !r1.contains(&field) && !r2.contains(&field) {
                    field_candidates[i].remove(name as &str);
                }
            }
        }
    }
    field_candidates
}

fn get_fields<'a>(candidates: &mut [HashSet<&'a str>]) -> Vec<&'a str> {
    while !candidates.iter().all(|c| c.len() == 1) {
        for i in 0..candidates.len() {
            let name = if candidates[i].len() == 1 {
                *candidates[i].iter().next().unwrap()
            } else {
                continue;
            };
            for j in (0..candidates.len()).filter(|&j| j != i) {
                candidates[j].remove(name);
            }
        }
    }
    candidates
        .iter()
        .map(|names| *names.iter().next().unwrap())
        .collect()
}

#[aoc(day16, part2)]
pub fn day16_part2(input: &Tickets) -> u64 {
    let fields = get_fields(&mut get_field_candidates(input));

    fields
        .iter()
        .enumerate()
        .filter_map(|(i, &name)| {
            if name.starts_with("departure") {
                Some(input.your_ticket[i])
            } else {
                None
            }
        })
        .product()
}
