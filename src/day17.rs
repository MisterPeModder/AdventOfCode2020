use std::{collections::HashMap, convert::Infallible, str::FromStr};

#[derive(Clone)]
pub struct Grid3 {
    data: HashMap<(i32, i32, i32), bool>,
    min_pos: (i32, i32, i32),
    max_pos: (i32, i32, i32),
}

impl Grid3 {
    pub fn from_active_positions(pos: HashMap<(i32, i32, i32), bool>) -> Self {
        let (min_pos, max_pos) = if pos.len() == 0 {
            ((0, 0, 0), (0, 0, 0))
        } else {
            pos.keys().fold(
                (
                    (i32::MAX, i32::MAX, i32::MAX),
                    (i32::MIN, i32::MIN, i32::MIN),
                ),
                |((min_x, min_y, min_z), (max_x, max_y, max_z)), &(x, y, z)| {
                    (
                        (min_x.min(x), min_y.min(y), min_z.min(z)),
                        (max_x.max(x), max_y.max(y), max_z.max(z)),
                    )
                },
            )
        };
        Grid3 {
            data: pos,
            min_pos,
            max_pos,
        }
    }

    #[inline]
    pub fn is_active_cube(&self, x: i32, y: i32, z: i32) -> bool {
        *self.data.get(&(x, y, z)).unwrap_or(&false)
    }

    pub fn next_cycle_cube(&self, x: i32, y: i32, z: i32) -> bool {
        match (self.is_active_cube(x, y, z), self.count_neighbors(x, y, z)) {
            (true, 2) | (true, 3) | (false, 3) => true,
            _ => false,
        }
    }

    fn count_neighbors(&self, x: i32, y: i32, z: i32) -> usize {
        #[rustfmt::skip]
        const NEIGHBORS: [(i32, i32, i32); 26] = [
            ( 1,  1,  1), ( 0,  1,  1), (-1,  1,  1),
            ( 1,  0,  1), ( 0,  0,  1), (-1,  0,  1),
            ( 1, -1,  1), ( 0, -1,  1), (-1, -1,  1),

            ( 1,  1,  0), ( 0,  1,  0), (-1,  1,  0),
            ( 1,  0,  0), /* center  */ (-1,  0,  0),
            ( 1, -1,  0), ( 0, -1,  0), (-1, -1,  0),

            ( 1,  1, -1), ( 0,  1, -1), (-1,  1, -1),
            ( 1,  0, -1), ( 0,  0, -1), (-1,  0, -1),
            ( 1, -1, -1), ( 0, -1, -1), (-1, -1, -1),
        ];

        NEIGHBORS
            .iter()
            .filter(|&&(dx, dy, dz)| self.is_active_cube(x + dx, y + dy, z + dz))
            .count()
    }
}

impl FromStr for Grid3 {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Grid3::from_active_positions(
            input
                .lines()
                .enumerate()
                .flat_map(|(y, l)| {
                    l.chars()
                        .enumerate()
                        .map(move |(x, c)| ((x as i32, y as i32, 0i32), c == '#'))
                })
                .collect(),
        ))
    }
}

#[derive(Clone)]
pub struct Grid4 {
    data: HashMap<(i32, i32, i32, i32), bool>,
    min_pos: (i32, i32, i32, i32),
    max_pos: (i32, i32, i32, i32),
}

impl Grid4 {
    pub fn from_active_positions(pos: HashMap<(i32, i32, i32, i32), bool>) -> Self {
        let (min_pos, max_pos) = if pos.len() == 0 {
            ((0, 0, 0, 0), (0, 0, 0, 0))
        } else {
            pos.keys().fold(
                (
                    (i32::MAX, i32::MAX, i32::MAX, i32::MAX),
                    (i32::MIN, i32::MIN, i32::MIN, i32::MIN),
                ),
                |((min_x, min_y, min_z, min_w), (max_x, max_y, max_z, max_w)), &(x, y, z, w)| {
                    (
                        (min_x.min(x), min_y.min(y), min_z.min(z), min_w.min(w)),
                        (max_x.max(x), max_y.max(y), max_z.max(z), max_w.max(w)),
                    )
                },
            )
        };
        Grid4 {
            data: pos,
            min_pos,
            max_pos,
        }
    }

    #[inline]
    pub fn is_active_cube(&self, x: i32, y: i32, z: i32, w: i32) -> bool {
        *self.data.get(&(x, y, z, w)).unwrap_or(&false)
    }

    pub fn next_cycle_cube(&self, x: i32, y: i32, z: i32, w: i32) -> bool {
        match (
            self.is_active_cube(x, y, z, w),
            self.count_neighbors(x, y, z, w),
        ) {
            (true, 2) | (true, 3) | (false, 3) => true,
            _ => false,
        }
    }

    fn count_neighbors(&self, x: i32, y: i32, z: i32, w: i32) -> usize {
        #[rustfmt::skip]
        const NEIGHBORS: [(i32, i32, i32, i32); 80] = [
            ( 1,  1,  1, -1), ( 0,  1,  1, -1), (-1,  1,  1, -1),
            ( 1,  0,  1, -1), ( 0,  0,  1, -1), (-1,  0,  1, -1),
            ( 1, -1,  1, -1), ( 0, -1,  1, -1), (-1, -1,  1, -1),

            ( 1,  1,  0, -1), ( 0,  1,  0, -1), (-1,  1,  0, -1),
            ( 1,  0,  0, -1), ( 0,  0,  0, -1), (-1,  0,  0, -1),
            ( 1, -1,  0, -1), ( 0, -1,  0, -1), (-1, -1,  0, -1),

            ( 1,  1, -1, -1), ( 0,  1, -1, -1), (-1,  1, -1, -1),
            ( 1,  0, -1, -1), ( 0,  0, -1, -1), (-1,  0, -1, -1),
            ( 1, -1, -1, -1), ( 0, -1, -1, -1), (-1, -1, -1, -1),

            ( 1,  1,  1,  0), ( 0,  1,  1,  0), (-1,  1,  1,  0),
            ( 1,  0,  1,  0), ( 0,  0,  1,  0), (-1,  0,  1,  0),
            ( 1, -1,  1,  0), ( 0, -1,  1,  0), (-1, -1,  1,  0),

            ( 1,  1,  0,  0), ( 0,  1,  0,  0), (-1,  1,  0,  0),
            ( 1,  0,  0,  0), /*   center   */  (-1,  0,  0,  0),
            ( 1, -1,  0,  0), ( 0, -1,  0,  0), (-1, -1,  0,  0),

            ( 1,  1, -1,  0), ( 0,  1, -1,  0), (-1,  1, -1,  0),
            ( 1,  0, -1,  0), ( 0,  0, -1,  0), (-1,  0, -1,  0),
            ( 1, -1, -1,  0), ( 0, -1, -1,  0), (-1, -1, -1,  0),

            ( 1,  1,  1,  1), ( 0,  1,  1,  1), (-1,  1,  1,  1),
            ( 1,  0,  1,  1), ( 0,  0,  1,  1), (-1,  0,  1,  1),
            ( 1, -1,  1,  1), ( 0, -1,  1,  1), (-1, -1,  1,  1),

            ( 1,  1,  0,  1), ( 0,  1,  0,  1), (-1,  1,  0,  1),
            ( 1,  0,  0,  1), ( 0,  0,  0,  1), (-1,  0,  0,  1),
            ( 1, -1,  0,  1), ( 0, -1,  0,  1), (-1, -1,  0,  1),

            ( 1,  1, -1,  1), ( 0,  1, -1,  1), (-1,  1, -1,  1),
            ( 1,  0, -1,  1), ( 0,  0, -1,  1), (-1,  0, -1,  1),
            ( 1, -1, -1,  1), ( 0, -1, -1,  1), (-1, -1, -1,  1),
        ];

        NEIGHBORS
            .iter()
            .filter(|&&(dx, dy, dz, dw)| self.is_active_cube(x + dx, y + dy, z + dz, w + dw))
            .count()
    }
}

impl FromStr for Grid4 {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Grid4::from_active_positions(
            input
                .lines()
                .enumerate()
                .flat_map(|(y, l)| {
                    l.chars()
                        .enumerate()
                        .map(move |(x, c)| ((x as i32, y as i32, 0i32, 0i32), c == '#'))
                })
                .collect(),
        ))
    }
}

#[aoc(day17, part1)]
pub fn day17_part1(input: &str) -> usize {
    let mut grid = Grid3::from_str(input).unwrap();

    for _ in 0..6 {
        let mut next = HashMap::with_capacity(grid.data.len());

        for x in (grid.min_pos.0 - 1)..=(grid.max_pos.0 + 1) {
            for y in (grid.min_pos.1 - 1)..=(grid.max_pos.1 + 1) {
                for z in (grid.min_pos.2 - 1)..=(grid.max_pos.2 + 1) {
                    next.insert((x, y, z), grid.next_cycle_cube(x, y, z));
                }
            }
        }
        grid = Grid3::from_active_positions(next);
    }
    grid.data.values().filter(|&&active| active).count()
}

#[aoc(day17, part2)]
pub fn day17_part2(input: &str) -> usize {
    let mut grid = Grid4::from_str(input).unwrap();

    for _ in 0..6 {
        let mut next = HashMap::with_capacity(grid.data.len());

        for x in (grid.min_pos.0 - 1)..=(grid.max_pos.0 + 1) {
            for y in (grid.min_pos.1 - 1)..=(grid.max_pos.1 + 1) {
                for z in (grid.min_pos.2 - 1)..=(grid.max_pos.2 + 1) {
                    for w in (grid.min_pos.3 - 1)..=(grid.max_pos.3 + 1) {
                        next.insert((x, y, z, w), grid.next_cycle_cube(x, y, z, w));
                    }
                }
            }
        }
        grid = Grid4::from_active_positions(next);
    }
    grid.data.values().filter(|&&active| active).count()
}
