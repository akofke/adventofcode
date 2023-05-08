use std::ops::RangeInclusive;

pub type Assignment = RangeInclusive<u32>;

pub fn parse(input: &str) -> Vec<(Assignment, Assignment)> {
    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(",").unwrap();
            let [first, second] = [first, second].map(|r| {
                let (begin, end) = r.split_once("-").unwrap();
                begin.parse().unwrap()..=end.parse().unwrap()
            });
            (first, second)
        })
        .collect()
}

pub fn part1(lines: &[(Assignment, Assignment)]) -> usize {
    lines
        .iter()
        .filter(|(first, second)| {
            first.contains(second.start()) && first.contains(second.end())
                || (second.contains(first.start()) && second.contains(first.end()))
        })
        .count()
}

pub fn part2(lines: &[(Assignment, Assignment)]) -> usize {
    lines
        .iter()
        .filter(|(first, second)| first.start() <= second.end() && first.end() >= second.start())
        .count()
}
