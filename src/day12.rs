#[aoc_generator(day12)]
pub fn day12_gen(input: &str) -> Vec<(char, i32)> {
    input
        .lines()
        .filter_map(|l| {
            if !l.is_char_boundary(1) {
                None
            } else {
                Some((l.chars().next()?, l[1..].parse::<i32>().ok()?))
            }
        })
        .collect()
}

#[aoc(day12, part1)]
pub fn day12_part1(input: &[(char, i32)]) -> u32 {
    const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let (_, x, y) = input.iter().copied().fold(
        (1usize, 0i32, 0i32),
        |(d, x, y), (action, value)| match action {
            'N' => (d, x, y + value),
            'S' => (d, x, y - value),
            'E' => (d, x + value, y),
            'W' => (d, x - value, y),
            'L' => (
                (d as isize - value as isize / 90).rem_euclid(4) as usize,
                x,
                y,
            ),
            'R' => (
                (d as isize + value as isize / 90).rem_euclid(4) as usize,
                x,
                y,
            ),
            'F' => (d, x + DIRECTIONS[d].0 * value, y + DIRECTIONS[d].1 * value),
            _ => (d, x, y),
        },
    );
    x.abs() as u32 + y.abs() as u32
}

#[aoc(day12, part2)]
pub fn day12_part2(input: &[(char, i32)]) -> u32 {
    type Rotation = fn((i32, i32)) -> (i32, i32);
    const ROTATIONS: [Rotation; 4] = [
        |(x, y)| (x, y),
        |(x, y)| (y, -x),
        |(x, y)| (-x, -y),
        |(x, y)| (-y, x),
    ];

    let ((x, y), _) = input.iter().copied().fold(
        ((0i32, 0i32), (10i32, 1i32)),
        |((x, y), (wx, wy)), (action, value)| match action {
            'N' => ((x, y), (wx, wy + value)),
            'S' => ((x, y), (wx, wy - value)),
            'E' => ((x, y), (wx + value, wy)),
            'W' => ((x, y), (wx - value, wy)),
            'L' => (
                (x, y),
                ROTATIONS[(-(value as isize) / 90).rem_euclid(4) as usize]((wx, wy)),
            ),
            'R' => (
                (x, y),
                ROTATIONS[(value as isize / 90).rem_euclid(4) as usize]((wx, wy)),
            ),
            'F' => ((x + wx * value, y + wy * value), (wx, wy)),
            _ => ((x, y), (wx, wy)),
        },
    );
    x.abs() as u32 + y.abs() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day12_part1() {
        assert_eq!(
            day12_part1(&[('F', 10), ('N', 3), ('F', 7), ('R', 90), ('F', 11)]),
            25
        );
    }

    #[test]
    fn test_day12_part2() {
        assert_eq!(
            day12_part2(&[('F', 10), ('N', 3), ('F', 7), ('R', 90), ('F', 11)]),
            286
        );
    }
}
