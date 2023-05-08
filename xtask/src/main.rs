use std::{env, io::Write};

use chrono::Datelike;
use reqwest::Method;

type DynErr = Box<dyn std::error::Error>;

const TEMPLATE: &'static str = r#"

pub fn parse(input: &str) -> Vec<i32> {
    unimplemented!()
}

pub fn part1(lines: &[i32]) -> usize {
    unimplemented!()
}

pub fn part2(lines: &[i32]) -> usize {
    unimplemented!()
}
"#;

fn main() -> Result<(), DynErr> {
    let args: Vec<_> = env::args().skip(1).collect();

    match args.as_slice() {
        &[ref year, ref day] => {
            let year = str::parse(year)?;
            let day = str::parse(day)?;
            new(year, day)?;
        }

        &[ref day] => {
            let day = str::parse(day)?;
            let year = chrono::Utc::now().year();
            new(year as u32, day)?;
        }

        _ => {
            todo!()
        }
    }
    Ok(())
}

fn new(year: u32, day: u32) -> Result<(), DynErr> {
    let sess = std::fs::read_to_string("C:/Users/.adventofcode.session")
        .or_else(|_| std::env::var("AOC_SESSION"))?;
    let client = reqwest::blocking::Client::new();
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let cookie = format!("session={sess}");
    let text = client
        .request(Method::GET, url)
        .header(reqwest::header::COOKIE, cookie)
        .send()?
        .text()?;

    let mut f = std::fs::File::create(format!("input/{year}/day{day:02}.txt"))?;
    f.write(text.as_bytes())?;

    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(format!("aoc/src/y{year}/day{day:02}.rs"))?;

    f.write_all(TEMPLATE.as_bytes())?;

    let mut f = std::fs::File::options()
        .append(true)
        .open(format!("aoc/src/y{year}/mod.rs"))?;
    f.write_all(format!("pub mod day{day:02};\n").as_bytes())?;
    Ok(())
}
