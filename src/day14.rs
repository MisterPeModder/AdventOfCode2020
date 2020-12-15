use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref MASK_PATTERN: Regex = Regex::new(r"^mask = ([01X]{36})$").unwrap();
    static ref VALUE_PATTERN: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
}

#[aoc(day14, part1)]
pub fn day14_part1(input: &str) -> u64 {
    let mut mem = HashMap::new();
    let mut positive = 0u64;
    let mut negative = 0u64;

    for line in input.lines() {
        if let Some(caps) = MASK_PATTERN.captures(line) {
            for c in caps[1].bytes() {
                match c {
                    b'0' => {
                        positive <<= 1;
                        negative <<= 1;
                    }
                    b'1' => {
                        positive = (positive << 1) | 1;
                        negative = (negative << 1) | 1;
                    }
                    b'X' => {
                        positive <<= 1;
                        negative = (negative << 1) | 1;
                    }
                    _ => unreachable!(),
                }
            }
        } else if let Some(caps) = VALUE_PATTERN.captures(line) {
            let k: usize = caps[1].parse().unwrap();
            let v: u64 = caps[2].parse().unwrap();

            mem.insert(k, (v | positive) & negative);
        }
    }
    mem.values().sum()
}

fn set_mem(mem: &mut HashMap<usize, u64>, index: usize, value: u64, mask: &[u8], i: u32) {
    if i == 36 {
        mem.insert(index, value);
        return;
    }
    match mask[i as usize] {
        b'0' => set_mem(mem, index, value, mask, i + 1),
        b'1' => set_mem(mem, index | (1 << (35 - i)), value, mask, i + 1),
        b'X' => {
            set_mem(mem, index | (1 << (35 - i)), value, mask, i + 1);
            set_mem(mem, index & !(1 << (35 - i)), value, mask, i + 1);
        }
        _ => unreachable!(),
    }
}

#[aoc(day14, part2)]
pub fn day14_part2(input: &str) -> u64 {
    let mut mem = HashMap::new();
    let mut mask: [u8; 36] = [0u8; 36];

    for line in input.lines() {
        if let Some(caps) = MASK_PATTERN.captures(line) {
            mask.copy_from_slice(caps[1].as_bytes());
        } else if let Some(caps) = VALUE_PATTERN.captures(line) {
            let k: usize = caps[1].parse().unwrap();
            let v: u64 = caps[2].parse().unwrap();

            set_mem(&mut mem, k, v, &mask, 0);
        }
    }
    mem.values().sum()
}
