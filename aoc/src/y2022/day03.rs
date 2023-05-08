use std::collections::HashSet;

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn item_value(item: u8) -> u8 {
    if item.is_ascii_uppercase() {
        item - b'A' + 1 + 26
    } else if item.is_ascii_lowercase() {
        item - b'a' + 1
    } else {
        panic!();
    }
}

pub fn part1(lines: &[&str]) -> u32 {
    let mut items = HashSet::new();
    lines
        .iter()
        .map(|line| {
            let mid = line.len() / 2;
            (&line[0..mid], &line[mid..])
        })
        .map(|(sack1, sack2)| {
            items.clear();
            items.extend(sack1.bytes());
            let common = sack2.bytes().filter(|b| items.contains(b)).next().unwrap();
            item_value(common) as u32
        })
        .sum()
}

pub fn part2(lines: &[&str]) -> u32 {
    lines
        .array_chunks()
        .map(|&[s1, s2, s3]| {
            let items: HashSet<_> = s1.bytes().collect();
            let items2 = s2.bytes().collect();
            let items: HashSet<_> = items.intersection(&items2).collect();
            let common = s3.bytes().filter(|c| items.contains(c)).next().unwrap();
            item_value(common) as u32
        })
        .sum()
}
