#![feature(array_windows)]
#![feature(array_chunks)]

use std::{borrow::Borrow, fmt::Display};

pub mod y2020;
pub mod y2021;
pub mod y2022;

pub trait Solution {
    type Parsed;
    type Input: Borrow<Self::Parsed>;
    type Output: Display;

    fn parse(input: &str) -> Self::Parsed;

    fn part1(input: Self::Input) -> Self::Output;
}

macro_rules! match_day {
    ($year_var:ident, $day_var:ident, $contents:expr, [ $($y:ident $d:ident ),* $(,)?]) => {
        match (format!("y{}", $year_var).as_str(), format!("day{:02}", $day_var).as_str()) {
            $(
                (stringify!($y), stringify!($d)) => {
                    let parsed = $crate::$y::$d::parse(&$contents);
                    let part1 = $crate::$y::$d::part1(&parsed);
                    println!("Part 1: {}", part1);
                    let part2 = $crate::$y::$d::part2(&parsed);
                    println!("Part 2: {}", part2);
                }
            )*
            _ => unimplemented!("Don't have {} day {} yet", $year_var, $day_var)
        }
    };
}

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();

    let day = str::parse(&args[0])?;
    let year = 2022;

    let contents = std::fs::read_to_string(format!("input/{year}/day{day:02}.txt"))?;
    run_day(year, day, contents);
    Ok(())
}

fn run_day(year: u32, day: u32, contents: String) {
    match_day!(year, day, contents, [
        y2020 day01,
        y2020 day02,
        y2020 day03,
        y2020 day04,
        y2020 day05,
        y2020 day06,
        y2020 day07,
        y2020 day08,
        y2020 day09,

        y2021 day01,
        y2021 day02,
        y2021 day03,
        y2021 day04,

        y2022 day01,
        y2022 day02,
        y2022 day03,
        y2022 day04,
        y2022 day05,
        y2022 day06,
    ]);
}
