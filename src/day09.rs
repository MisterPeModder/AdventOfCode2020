use std::{cmp::Ordering, collections::VecDeque};

fn find_invalid_num(nums: &[u32]) -> u32 {
    let mut preamble: VecDeque<_> = nums.iter().take(25).copied().collect();

    for n in nums.iter().skip(25).copied() {
        if !preamble
            .iter()
            .any(|&p| p < n && preamble.contains(&(n - p)))
        {
            return n;
        }
        preamble.pop_front();
        preamble.push_back(n);
    }
    panic!("cannot find number")
}

#[aoc(day09, part1)]
pub fn day09_part1(input: &str) -> u32 {
    let nums: Vec<_> = input
        .lines()
        .filter_map(|l| l.parse::<u32>().ok())
        .collect();

    find_invalid_num(&nums)
}

#[aoc(day09, part2)]
pub fn day09_part2(input: &str) -> u32 {
    let nums: Vec<_> = input
        .lines()
        .filter_map(|l| l.parse::<u32>().ok())
        .collect();
    let invalid = find_invalid_num(&nums);
    let nums: Vec<_> = nums.into_iter().filter(|&n| n != invalid).collect();

    (0..nums.len())
        .find_map(|start| {
            nums.iter()
                .skip(start)
                .try_fold(
                    (u32::MAX, u32::MIN, 0u32),
                    |(smallest, largest, sum), &n| match (sum + n).cmp(&invalid) {
                        Ordering::Less => Ok((smallest.min(n), largest.max(n), sum + n)),
                        Ordering::Greater => Ok((u32::MAX, u32::MIN, 0)),
                        Ordering::Equal => Err(smallest.min(n) + largest.max(n)),
                    },
                )
                .err()
        })
        .unwrap()
}
