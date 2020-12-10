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
        .collect();
    adapters.sort_unstable();

    let last = *adapters.last().unwrap();
    let mut ways: Vec<u64> = iter::repeat(0).take(last as usize + 1).collect();
    let mut adapters = adapters.into_iter().peekable();

    ways[0] = 1;
    if let Some(1) = adapters.peek().copied() {
        ways[1] = ways[0];
        adapters.next();
        if let Some(2) = adapters.peek().copied() {
            ways[2] = ways[1] + ways[0];
            adapters.next();
        }
    }
    for n in adapters {
        ways[n as usize] = ways[n as usize - 1] + ways[n as usize - 2] + ways[n as usize - 3];
    }
    ways[last as usize]
}
