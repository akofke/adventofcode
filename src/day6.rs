use std::collections::HashSet;

pub fn parse(input: &str) -> Vec<&str> {
    input
        .split("\n\n")
        .collect()
}

pub fn part1(input: &[&str]) -> usize {
    input
        .iter()
        .map(|group| {
            let answers: HashSet<_> = group
                .chars()
                .filter(|&c| c != '\n')
                .collect();
            answers.len()
        })
        .sum()
}

pub fn part2(input: &[&str]) -> usize {
    input
        .iter()
        .map(|group| {
            group
                .lines()
                .map(|line| line.chars().collect::<HashSet<_>>())
                .fold_first(|set, next_set| set.intersection(&next_set).copied().collect())
                .unwrap()
                .len()
        })
        .sum()
}