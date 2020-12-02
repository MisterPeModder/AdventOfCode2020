use std::collections::HashSet;

#[aoc_generator(day01)]
pub fn day01_gen(input: &str) -> HashSet<u32> {
    input.lines().map(|l| l.parse().expect("not an int")).collect()
}

const TARGET_SUM: u32 = 2020;

fn get_matching_entry(sum: u32, m: u32, entries: &HashSet<u32>) -> Option<u32> {
    if m > sum {
        return None;
    }
    let n = sum - m;

    if m == n {
        None
    } else {
        entries.get(&n).map(|_| n)
    }
}

#[inline]
fn get_matching_pair(sum: u32, entries: &HashSet<u32>) -> Option<(u32, u32)> {
    entries.iter().find_map(|&m| get_matching_entry(sum, m, entries).map(|n| (m, n)))
}

fn get_matching_triple(sum: u32, entries: &HashSet<u32>) -> Option<(u32, u32, u32)> {
    entries
        .iter()
        .filter_map(|&m| if m > sum {
            None
        } else {
            let n_o = sum - m;

            if m == n_o {
                None
            } else {
                Some((m, n_o))
            }
        })
        .find_map(|(m, n_o)| get_matching_pair(n_o, entries)
            .map(|(n, o)| (m, n, o)))
}

#[aoc(day01, part1)]
pub fn day01_part1(input: &HashSet<u32>) -> u32 {
    let (m, n) = get_matching_pair(TARGET_SUM, input)
        .expect("cannot find matching pair");

    m * n
}

#[aoc(day01, part2)]
pub fn day01_part2(input: &HashSet<u32>) -> u32 {
    let (m, n, o) = get_matching_triple(TARGET_SUM, input)
        .expect("cannot find matching triple");


    m * n * o
}