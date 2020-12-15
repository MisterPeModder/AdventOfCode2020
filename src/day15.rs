fn nth_spoken_number(first_nums: &[u32], n: u32) -> u32 {
    let mut spoken = first_nums[first_nums.len() - 1];
    let mut nums: Vec<Option<(u32, u32)>> =
        vec![None; first_nums.iter().copied().max().unwrap() as usize + 1];

    first_nums
        .iter()
        .enumerate()
        .for_each(|(i, &n)| nums[n as usize] = Some((i as u32, i as u32)));
    for turn in first_nums.len() as u32..n {
        let prev_turns = nums[spoken as usize].unwrap();

        spoken = prev_turns.1 - prev_turns.0;
        nums.resize(nums.len().max(spoken as usize + 1), None);
        nums[spoken as usize] = match nums[spoken as usize] {
            Some((_, t)) => Some((t, turn)),
            None => Some((turn, turn)),
        };
    }
    spoken
}

#[aoc_generator(day15)]
pub fn day15_gen(input: &str) -> Vec<u32> {
    input.split(',').filter_map(|n| n.parse().ok()).collect()
}

#[aoc(day15, part1)]
pub fn day15_part1(input: &[u32]) -> u32 {
    nth_spoken_number(&input, 2020)
}

#[aoc(day15, part2)]
pub fn day15_part2(input: &[u32]) -> u32 {
    nth_spoken_number(&input, 30000000)
}
