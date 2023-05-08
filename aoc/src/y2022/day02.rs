use std::array;

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl RPS {
    pub fn shape_score(self) -> u32 {
        self as u32
    }

    fn outcome_with(self, other: Self) -> Outcome {
        use RPS::*;
        match (self, other) {
            (s1, s2) if s1 == s2 => Outcome::Draw,
            (Rock, Scissors) => Outcome::Win,
            (Rock, Paper) => Outcome::Loss,
            (Paper, Scissors) => Outcome::Loss,
            (_, _) => other.outcome_with(self).invert(),
        }
    }

    fn need_for_outcome(self, outcome: Outcome) -> Self {
        match (self, outcome) {
            (_, Outcome::Draw) => self,
            (RPS::Rock, Outcome::Win) => RPS::Paper,
            (RPS::Paper, Outcome::Win) => RPS::Scissors,
            (RPS::Scissors, Outcome::Win) => RPS::Rock,
            (RPS::Rock, Outcome::Loss) => RPS::Scissors,
            (RPS::Paper, Outcome::Loss) => RPS::Rock,
            (RPS::Scissors, Outcome::Loss) => RPS::Paper,
        }
    }

    fn from_str(s: &str) -> Self {
        match s {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scissors,
            _ => panic!("not allowed"),
        }
    }
}

#[derive(Clone, Copy)]
#[repr(u32)]
enum Outcome {
    Win = 6,
    Loss = 0,
    Draw = 3,
}

impl Outcome {
    fn points(self) -> u32 {
        self as u32
    }

    fn invert(self) -> Self {
        match self {
            Self::Win => Self::Loss,
            Self::Loss => Self::Win,
            _ => Self::Draw,
        }
    }

    fn from_str(s: &str) -> Self {
        match s {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("not allowed"),
        }
    }
}

pub fn parse(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect_tuple().unwrap())
        .collect()
}

pub fn part1(lines: &[(&str, &str)]) -> u32 {
    lines
        .iter()
        .map(|&(theirs, yours)| (RPS::from_str(theirs), RPS::from_str(yours)))
        .map(|(theirs, yours)| yours.outcome_with(theirs).points() + yours.shape_score())
        .sum()
}

pub fn part2(lines: &[(&str, &str)]) -> u32 {
    lines
        .iter()
        .map(|&(theirs, outcome)| (RPS::from_str(theirs), Outcome::from_str(outcome)))
        .map(|(theirs, outcome)| {
            RPS::need_for_outcome(theirs, outcome).shape_score() + outcome.points()
        })
        .sum()
}
