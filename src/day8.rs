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
    0
}