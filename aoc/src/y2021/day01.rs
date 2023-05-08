

pub fn parse(input: &str) -> Vec<i32> {
    input.lines()
    .map(|line| str::parse(line).unwrap())
    .collect()
}

pub fn part1(lines: &[i32]) -> usize {
    lines
        .array_windows()
        .filter(|[a, b]| b > a)
        .count()
}

pub fn part2(lines: &[i32]) -> usize {
    lines
        .array_windows()
        .map(|[a, b, c]| a + b + c)
        .collect::<Vec<_>>()
        .array_windows()
        .filter(|[a, b]| b > a)
        .count()
}
