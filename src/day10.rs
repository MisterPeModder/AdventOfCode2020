use std::{collections::HashMap, iter};

#[aoc(day10, part1)]
pub fn day10_part1(input: &str) -> u32 {
    let mut adapters: Vec<u32> = input.lines().filter_map(|l| l.parse().ok()).collect();

    adapters.sort_unstable();
    let (_, d1, d3) = adapters.iter().fold((0, 0, 0), |(prev, d1, d3), &n| {
        let diff = n - prev;

        if diff == 1 {
            (n, d1 + 1, d3)
        } else if diff == 3 {
            (n, d1, d3 + 1)
        } else {
            (n, d1, d3)
        }
    });
    d1 * (d3 + 1)
}

#[aoc(day10, part2)]
pub fn day10_part2(input: &str) -> u64 {
    let mut adapters: Vec<u32> = input
        .lines()
        .filter_map(|l| l.parse().ok())
        .chain(iter::once(0))
        .collect();
    let mut ways: HashMap<u32, u64> = HashMap::new();
    let last;

    adapters.sort_unstable();
    last = *adapters.last().unwrap();
    ways.insert(0, 1);
    for n in adapters.into_iter().skip(1) {
        ways.insert(
            n,
            ways.get(&(n - 1)).copied().unwrap_or_default()
                + ways.get(&(n - 2)).copied().unwrap_or_default()
                + ways.get(&(n - 3)).copied().unwrap_or_default(),
        );
    }
    ways[&last]
}
