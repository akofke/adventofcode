
pub fn parse(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part1(input: &[i64]) -> i64 {
    input
        .array_windows::<26>()
        .find(|window| !window_valid(window))
        .unwrap()[25]
}

fn window_valid(window: &[i64; 26]) -> bool {
    let target = window[25];
    let window = &window[..25];
    for x in window {
        let y = target - x;
        if window.iter().find(|&&v| v == y).is_some() {
            return true
        }
    }
    false
}

pub fn part2(input: &[i64]) -> i64 {
    let target = part1(input);
    (2..input.len())
        .flat_map(|window_len| {
            input.windows(window_len)
        })
        .find(|window| window.iter().sum::<i64>() == target)
        .map(|window| {
            let min = window.iter().min().unwrap();
            let max = window.iter().max().unwrap();
            min + max
        })
        .unwrap()
}