use itertools::Itertools;
pub type Input = (Vec<Vec<u8>>, Vec<(usize, usize, usize)>);

pub fn parse(input: &str) -> Input {
    let (stacks_str, moves) = input.split_once("\n\n").unwrap();

    let mut stacks: Vec<Vec<u8>> = stacks_str
        .lines()
        .rev()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|_| Vec::new())
        .collect();

    for line in stacks_str.lines().rev().skip(1) {
        for (item, stack) in line.as_bytes().chunks(4).zip(&mut stacks) {
            if item[1].is_ascii_alphabetic() {
                stack.push(item[1])
            }
        }
    }

    let moves = moves
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect_tuple()
                .unwrap()
        })
        .collect();
    (stacks, moves)
}

pub fn part1(input: &Input) -> String {
    let (stacks, moves) = input;
    let mut stacks = stacks.clone();
    let mut tmp = Vec::new();
    for (num, from, to) in moves {
        let stack = &mut stacks[from - 1];
        tmp.extend(stack.drain(stack.len() - num..).rev());
        (&mut stacks[to - 1]).extend(tmp.drain(..));
    }
    let top = stacks
        .iter_mut()
        .map(|stack| stack.pop().unwrap())
        .collect();

    String::from_utf8(top).unwrap()
}

pub fn part2(input: &Input) -> String {
    let (stacks, moves) = input;
    let mut stacks = stacks.clone();
    let mut tmp = Vec::new();
    for (num, from, to) in moves {
        let stack = &mut stacks[from - 1];
        tmp.extend(stack.drain(stack.len() - num..));
        (&mut stacks[to - 1]).extend(tmp.drain(..));
    }
    let top = stacks
        .iter_mut()
        .map(|stack| stack.pop().unwrap())
        .collect();

    String::from_utf8(top).unwrap()
}
