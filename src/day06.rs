use std::collections::HashSet;

#[aoc(day06, part1)]
pub fn day06_part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|g| {
            g.lines()
                .flat_map(|l| l.chars())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

#[aoc(day06, part2)]
pub fn day06_part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|g| {
            let mut lines = g.lines();
            let map: HashSet<_> = lines.next().unwrap().chars().collect();

            lines
                .fold(map, |map, l| {
                    map.intersection(&l.chars().collect()).copied().collect()
                })
                .len()
        })
        .sum()
}
