pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> usize {
    let (i, _) = input
        .as_bytes()
        .array_windows::<4>()
        .enumerate()
        .find(|&(i, &window)| {
            let mut set = 0u128;
            for code in window {
                set ^= 1u128 << code;
            }
            set.count_ones() == 4
        })
        .unwrap();

    i + 4
}

pub fn part2(input: &str) -> usize {
    let (i, _) = input
        .as_bytes()
        .array_windows::<14>()
        .enumerate()
        .find(|&(i, &window)| {
            let mut set = 0u128;
            for code in window {
                set ^= 1u128 << code;
            }
            set.count_ones() == 14
        })
        .unwrap();

    i + 14
}
