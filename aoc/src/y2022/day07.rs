use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, digit1, not_line_ending, space1},
    combinator::{eof, recognize},
    sequence::{separated_pair, terminated},
    IResult, Parser,
};
use petgraph::prelude::DiGraph;

#[derive(PartialEq, Eq, Clone)]
pub enum Node {
    Dir(String),
    File(String, usize),
}

enum Command {
    Ls,
    CdUp,
    Cd(String),
}

fn command(input: &str) -> IResult<&str, Command> {
    let (input, _) = terminated(tag("$"), space1)(input)?;
    let ls = tag("ls").map(|_| Command::Ls);
    let cdup = separated_pair(tag("cd"), space1, tag("..")).map(|_| Command::CdUp);
    let cd = separated_pair(tag("cd"), space1, tag(".."))
        .map(|(_, dirname): (&str, &str)| Command::Cd(dirname.to_string()));
    alt((ls, cdup, cd))(input)
}

fn node(input: &str) -> IResult<&str, Node> {
    let dir = separated_pair(tag("dir"), space1, eof);
    let file = separated_pair(digit1, space1, eof);
    alt((
        dir.map(|(_, dirname): (&str, &str)| Node::Dir(dirname.to_string())),
        file.map(|(size, name): (&str, &str)| Node::File(name.to_string(), size.parse().unwrap())),
    ))(input)
}

pub fn parse(input: &str) -> DiGraph<Node, ()> {
    let mut lines = input.lines();

    unimplemented!()
}

pub fn part1(lines: &[i32]) -> usize {
    unimplemented!()
}

pub fn part2(lines: &[i32]) -> usize {
    unimplemented!()
}
