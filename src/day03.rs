use std::{
    iter,
    str::FromStr,
};

pub struct Map {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Map {
    #[inline]
    fn is_tree(&self, x: usize, y: usize) -> bool {
        self.data[x + y * self.width] != 0
    }

    pub fn count_trees(&self, slope_x: usize, slope_y: usize) -> usize {
        let mut x = 0;
        let mut count = 0;

        for y in (0..self.height).step_by(slope_y) {
            count += self.is_tree(x, y) as usize;
            x = (x + slope_x) % self.width;
        }
        count
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();
        let width = lines.peek().copied().unwrap_or("").as_bytes().len();
        let data: Vec<u8> = lines.flat_map(|l| l
                .bytes()
                .chain(iter::repeat(b'.'))
                .take(width)
                .map(|c| (c == b'#') as u8)).collect();
        let height = data.len() / width;
        
        Ok(Map { data, width, height })
    }
}

#[aoc_generator(day03)]
pub fn day03_gen(input: &str) -> Map {
    Map::from_str(input).unwrap()
}

#[aoc(day03, part1)]
pub fn day02_part1(map: &Map) -> usize {
    map.count_trees(3, 1)
}

#[aoc(day03, part2)]
pub fn  day02_part2(map: &Map) -> usize {
    const SLOPES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    SLOPES.iter().map(|&(x, y)| map.count_trees(x, y)).product()
}