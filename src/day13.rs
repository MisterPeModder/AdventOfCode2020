#[aoc(day13, part1)]
pub fn aoc_day13_part1(input: &str) -> u32 {
    input
        .split_once('\n')
        .and_then(|(time, ids)| {
            let time: u32 = time.parse().ok()?;
            let (id, diff) = ids.split(',').filter_map(|id| id.parse().ok()).fold(
                (u32::MAX, u32::MAX),
                |(earliest, diff), id| {
                    let d = (time / id + 1) * id - time;
                    if d < diff {
                        (id, d)
                    } else {
                        (earliest, diff)
                    }
                },
            );

            Some(id * diff)
        })
        .unwrap()
}

#[aoc(day13, part2)]
pub fn aoc_day13_part2(input: &str) -> u64 {
    input
        .split_once('\n')
        .and_then(|(_, ids)| {
            Some(
                ids.split(',')
                    .zip(0u64..)
                    .filter_map(|(id, pos)| Some((pos, id.parse::<u64>().ok()?)))
                    .fold((0u64, 1u64), |(min, product), (pos, id)| {
                        (
                            (min..)
                                .step_by(product as usize)
                                .find(|&min| (min + pos) % id == 0)
                                .unwrap(),
                            product * id,
                        )
                    })
                    .0,
            )
        })
        .unwrap()
}
