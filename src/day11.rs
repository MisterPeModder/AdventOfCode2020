use std::collections::HashMap;

fn is_occupied(state: &HashMap<(i32, i32), bool>, x: i32, y: i32) -> bool {
    *state.get(&(x, y)).unwrap_or(&false)
}

fn is_first_occupied(
    state: &HashMap<(i32, i32), bool>,
    width: i32,
    height: i32,
    mut x: i32,
    mut y: i32,
    dx: i32,
    dy: i32,
) -> bool {
    x += dx;
    y += dy;
    while x >= 0 && x <= width && y >= 0 && y <= height {
        if let Some(&occupied) = state.get(&(x, y)) {
            return occupied;
        }
        x += dx;
        y += dy;
    }
    false
}

#[aoc(day11, part1)]
pub fn day11_part1(input: &str) -> usize {
    let mut state: HashMap<(i32, i32), bool> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.bytes().enumerate().filter_map(move |(x, c)| match c {
                b'L' => Some(((x as i32, y as i32), false)),
                _ => None,
            })
        })
        .collect();
    loop {
        let new_state: HashMap<(i32, i32), bool> = state
            .iter()
            .map(|(&(x, y), &occupied)| {
                let seats = is_occupied(&state, x - 1, y - 1) as u32
                    + is_occupied(&state, x, y - 1) as u32
                    + is_occupied(&state, x + 1, y - 1) as u32
                    + is_occupied(&state, x - 1, y) as u32
                    + is_occupied(&state, x + 1, y) as u32
                    + is_occupied(&state, x - 1, y + 1) as u32
                    + is_occupied(&state, x, y + 1) as u32
                    + is_occupied(&state, x + 1, y + 1) as u32;

                if occupied {
                    ((x, y), seats < 4)
                } else {
                    ((x, y), seats == 0)
                }
            })
            .collect();

        if new_state == state {
            break state.values().filter(|&&o| o).count();
        } else {
            state = new_state;
        }
    }
}

#[aoc(day11, part2)]
pub fn day11_part2(input: &str) -> usize {
    let mut width = 0;
    let mut height = 0;
    let mut state: HashMap<(i32, i32), bool> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            width = width.max(l.len() as i32);
            height = height.max(y as i32);
            l.bytes().enumerate().filter_map(move |(x, c)| match c {
                b'L' => Some(((x as i32, y as i32), false)),
                _ => None,
            })
        })
        .collect();
    loop {
        let new_state: HashMap<(i32, i32), bool> = state
            .iter()
            .map(|(&(x, y), &occupied)| {
                let seats = is_first_occupied(&state, width, height, x, y, -1, -1) as u32
                    + is_first_occupied(&state, width, height, x, y, 0, -1) as u32
                    + is_first_occupied(&state, width, height, x, y, 1, -1) as u32
                    + is_first_occupied(&state, width, height, x, y, -1, 0) as u32
                    + is_first_occupied(&state, width, height, x, y, 1, 0) as u32
                    + is_first_occupied(&state, width, height, x, y, -1, 1) as u32
                    + is_first_occupied(&state, width, height, x, y, 0, 1) as u32
                    + is_first_occupied(&state, width, height, x, y, 1, 1) as u32;

                if occupied {
                    ((x, y), seats < 5)
                } else {
                    ((x, y), seats == 0)
                }
            })
            .collect();

        if new_state == state {
            break state.values().filter(|&&o| o).count();
        } else {
            state = new_state;
        }
    }
}
