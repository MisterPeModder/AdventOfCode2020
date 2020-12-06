use std::str::FromStr;

pub struct Seat {
    id: u32,
}

impl Seat {
    pub fn new(row: u32, col: u32) -> Self {
        Seat { id: row * 8 + col }
    }

    fn bsp(pattern: &[u8], lower: u8, upper: u8) -> Option<u32> {
        pattern.iter().try_fold(0, |acc, &c| match c {
            _ if c == lower => Some(acc << 1),
            _ if c == upper => Some(acc << 1 | 1),
            _ => None,
        })
    }
}

impl FromStr for Seat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = s.as_bytes();

        match (b.get(0..7), b.get(7..10), b.get(10..)) {
            (Some(row), Some(col), Some(&[])) => {
                match (Self::bsp(row, b'F', b'B'), Self::bsp(col, b'L', b'R')) {
                    (Some(row), Some(col)) => Ok(Self::new(row, col)),
                    _ => Err(()),
                }
            }
            _ => Err(()),
        }
    }
}

#[aoc_generator(day05)]
pub fn day05_gen(input: &str) -> Vec<Seat> {
    let mut i: Vec<Seat> = input
        .lines()
        .filter_map(|l| Seat::from_str(l).ok())
        .collect();
    i.sort_unstable_by_key(|s| s.id);
    i
}

#[aoc(day05, part1)]
pub fn day05_part1(input: &[Seat]) -> u32 {
    input[input.len() - 1].id
}

#[aoc(day05, part2)]
pub fn day05_part2(input: &[Seat]) -> u32 {
    input.windows(2).find(|&w| w[0].id + 1 != w[1].id).unwrap()[0].id + 1
}
