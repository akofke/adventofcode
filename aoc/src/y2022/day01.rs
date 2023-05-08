use itertools::Itertools;

pub fn parse(input: &str) -> Vec<Option<u32>> {
    input.lines().map(|line| line.parse().ok()).collect()
}

pub fn part1(lines: &[Option<u32>]) -> u32 {
    lines
        .iter()
        .fold((0, 0), |(largest, sum), elem| match *elem {
            Some(val) => (largest, sum + val),
            None => {
                if sum > largest {
                    (sum, 0)
                } else {
                    (largest, 0)
                }
            }
        })
        .0
}

pub fn part2(lines: &[Option<u32>]) -> u32 {
    lines
        .iter()
        .copied()
        .coalesce(|sum, next| match (sum, next) {
            (Some(sum), Some(next)) => Ok(Some(sum + next)),
            (Some(sum), None) => Err((Some(sum), Some(0))),
            _ => unreachable!(),
        })
        .flatten()
        .sorted()
        .rev()
        .take(3)
        .sum()
}

// lol... could have just used slice::split
