use parse_display::{FromStr, Display};

#[derive(Display, FromStr, Copy, Clone, Debug)]
#[display(style = "lowercase")]
pub enum Instr {
    #[display("{} {0}")]
    Nop(i32),

    #[display("{} {0}")]
    Acc(i32),

    #[display("{} {0}")]
    Jmp(i32),
}

impl Instr {
    pub fn can_swap(&self) -> bool {
        matches!(self, Instr::Nop(_) | Instr::Jmp(_))
    }
    pub fn swap_instr(&self) -> Self {
        match *self {
            Instr::Nop(v) => { Instr::Jmp(v) }
            acc @ Instr::Acc(_) => { acc }
            Instr::Jmp(v) => { Instr::Nop(v) }
        }
    }
}

pub fn parse(input: &str) -> Vec<Instr> {
    input.lines().map(|line| dbg!(line.parse()).unwrap()).collect()
}

pub fn part1(input: &[Instr]) -> i32 {
    let mut visited = vec![false; input.len()];
    let mut pc = 0;
    let mut accum = 0i32;

    while !visited[pc] {
        visited[pc] = true;
        match input[pc] {
            Instr::Nop(_) => {
                pc += 1;
            }

            Instr::Acc(val) => {
                accum += val;
                pc += 1;
            }

            Instr::Jmp(offset) => {
                pc = (pc as i32 + offset) as usize
            }
        }
    }
    accum
}

pub fn part2(input: &[Instr]) -> i32 {
    let mut input = input.to_vec();
    let mut visited = vec![false; input.len()];
    for i in 0..input.len() {
        let instr = input[i];
        if instr.can_swap() {
            input[i] = instr.swap_instr();
            if let Some(res) = run_program(&input, &mut visited) {
                return res;
            }
            input[i] = instr;
        }
    }
    panic!("couldn't find answer")
}

fn run_program(input: &[Instr], visited: &mut [bool]) -> Option<i32> {
    visited.fill(false);
    let mut pc = 0;
    let mut accum = 0i32;

    loop {
        if pc == input.len() { break Some(accum) }
        if visited[pc] { break None }
        visited[pc] = true;
        match input[pc] {
            Instr::Nop(_) => {
                pc += 1;
            }

            Instr::Acc(val) => {
                accum += val;
                pc += 1;
            }

            Instr::Jmp(offset) => {
                pc = (pc as i32 + offset) as usize
            }
        }
    }
}