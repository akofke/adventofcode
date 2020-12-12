use parse_display::FromStr;


pub fn parse(input: &str) -> Vec<PassRule> {
    input
        .lines()
        .map(|line| {
            line.parse().expect("Can't parse line")
        })
        .collect()
}

#[derive(FromStr)]
#[display("{i1}-{i2} {letter}: {password}")]
pub struct PassRule {
    i1: u32,
    i2: u32,
    letter: char,
    password: String,
}

impl PassRule {
    pub fn valid_part1(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.letter).count() as u32;
        (self.i1..=self.i2).contains(&count)
    }

    pub fn valid_part2(&self) -> bool {
        let bytes = self.password.as_bytes();
        (bytes[self.i1 as usize - 1] as char == self.letter) ^ (bytes[self.i2 as usize - 1] as char == self.letter)
    }
}

pub fn part1(input: &[PassRule]) -> usize {
    input.iter().filter(|pass| pass.valid_part1()).count()
}

pub fn part2(input: &[PassRule]) -> usize {
    input.iter().filter(|pass| pass.valid_part2()).count()
}

pub fn run() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("input/2020/day2.txt")?;
    let parsed = parse(&contents);
    println!("{}", part1(&parsed));
    println!("{}", part2(&parsed));
    Ok(())
}