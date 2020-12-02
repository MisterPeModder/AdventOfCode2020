#[aoc_generator(day01)]
pub fn day01_gen(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().expect("not an int")).collect()
}

#[aoc(day01, part1)]
pub fn day01_part1(input: &[i32]) -> i32 {
    let len = input.len();

    if len < 2 {
        0
    } else {
        for (i, &n1) in input[..len - 1].iter().enumerate() {
            for &n2 in input[i + 1..].iter() {
                if n1 + n2 == 2020 {
                    return n1 * n2;
                }
            }
        }
        0
    }
}

#[aoc(day01, part2)]
pub fn day01_part2(input: &[i32]) -> i32 {
    let len = input.len();

    if len < 3 {
        0
    } else {
        for (i, &n1) in input[..len - 2].iter().enumerate() {
            for &n2 in input[i + 1..].iter() {
                for &n3 in input[i + 2..].iter() {
                    if n1 + n2 + n3 == 2020 {
                        return n1 * n2 * n3;
                    }
                }
            }
        }
        0
    }
}