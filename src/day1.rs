use std::collections::HashSet;

pub fn parse(input: &str) -> HashSet<i32> {
    input.lines().map(|line| line.parse::<i32>().unwrap()).collect()
}

pub fn part1(numbers: &HashSet<i32>) -> i32 {
    numbers
        .iter()
        .find_map(|&x| {
            let partner = 2020 - x;
            numbers.get(&partner).map(|&y| y * x)
        })
        .expect("Couldn't find pair")
}

pub fn part2(numbers: &HashSet<i32>) -> i32 {
    numbers
        .iter()
        .find_map(|&x| {
            numbers.iter().find_map(|&y| {
                let target = 2020 - x - y;
                numbers.get(&target).map(|&z| x * y * z)
            })
        })
        .expect("Couldn't find values")
}

pub fn run() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("input/2020/day1.txt")?;
    let input = parse(&contents);
    println!("{}", part1(&input));
    println!("{}", part2(&input));
    Ok(())
}