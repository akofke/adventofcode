use std::collections::{HashMap};

use regex::Regex;


pub fn parse(input: &str) -> Vec<HashMap<&str, &str>> {
    input
        .split("\n\n")
        .map(|passport| {
            passport
                .split_ascii_whitespace()
                .map(|field| {
                    field.split_once(":").expect("Can't parse field")
                })
                .collect::<HashMap<_, _>>()
        })
        .collect()
}

const EXPECTED_FIELDS: [&str; 7] = [
        "byr", 
        "iyr", 
        "eyr", 
        "hgt", 
        "hcl", 
        "ecl", 
        "pid", 
    ];


pub fn part1(input: &[HashMap<&str, &str>]) -> usize {
    input
        .iter()
        .filter(|passport| {
            EXPECTED_FIELDS.iter().all(|k| passport.contains_key(k))
        })
        .count()
}

pub fn part2(input: &[HashMap<&str, &str>]) -> usize {
    input
        .iter()
        .filter(|passport| {
            EXPECTED_FIELDS.iter().all(|k| passport.contains_key(k))
            && passport.iter().all(|(k, v)| {
                validate_field(k, v)
            })
        })
        .count()
}

fn validate_field(key: &str, value: &str) -> bool {
    match key {
        "byr" => {
            let year = value.parse::<u32>().ok();
            year.filter(|&year| year >= 1920 && year <= 2002).is_some()
        }
        "iyr" => {
            value.parse::<u32>().ok().filter(|&year| year >= 2010 && year <= 2020).is_some()
        }
        "eyr" => {
            value.parse::<u32>().ok().filter(|&year| year >= 2020 && year <= 2030).is_some()
        }
        "hgt" => {
            let re = Regex::new(r"(\d+)(\w+)").unwrap();
            if let Some(caps) = re.captures(value) {
                match (&caps[1], &caps[2]) {
                    (num, "cm") => {
                        num.parse::<u32>().ok().filter(|&ht| ht >= 150 && ht <= 193).is_some() 
                    }
                    (num, "in") => {
                        num.parse::<u32>().ok().filter(|&ht| ht >= 59 && ht <= 76).is_some()
                    }
                    _ => false
                }
            } else {
                false
            }
        }
        "hcl" => {
            let re = Regex::new(r"#[0-9a-f]{6}").unwrap();
            re.is_match(value)
        }
        "ecl" => {
            match value {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false
            }
        }
        "pid" => {
            value.len() == 9 && value.chars().all(|c| c.is_ascii_digit())
        }
        "cid" => true,
        _ => {
            dbg!(key);
            false
        }
    }
}