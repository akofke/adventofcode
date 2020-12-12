use nom::{IResult, Parser};
use nom::sequence::{terminated, separated_pair};
use nom::bytes::complete::{take_until, tag};
use nom::combinator::{opt, all_consuming};
use nom::character::complete::digit1;
use nom::branch::alt;
use nom::multi::separated_list1;
use petgraph::graphmap::DiGraphMap;
use petgraph::visit::{Dfs, Reversed};

pub fn parse(input: &str) -> DiGraphMap<&str, u32> {
    let mut graph = DiGraphMap::new();
    input
        .lines()
        .map(|line| {
            bag_spec(line)
        })
        .for_each(|(bag, bag_spec)| {
            if let Some(v) = bag_spec {
                for (count, inner_bag) in v {
                    graph.add_edge(bag, inner_bag, count);
                }
            } else {
                graph.add_node(bag);
            }
        });
    return graph
}

pub fn part1(input: &DiGraphMap<&str, u32>) -> usize {
    // println!("{:?}", petgraph::dot::Dot::new(input));
    let reversed = Reversed(input);
    let mut dfs = Dfs::new(reversed, "shiny gold");
    let mut count = 0;
    while let Some(n) = dfs.next(reversed) {
        count += 1;
    }
    count - 1
}

pub fn part2(input: &DiGraphMap<&str, u32>) -> usize {
    0
}

fn bag(input: &str) -> IResult<&str, &str> {
    terminated(
        take_until(" bag"),
        terminated(tag(" bag"), opt(tag("s")))
    )(input)
}

fn bag_count(input: &str) -> IResult<&str, (u32, &str)> {
    separated_pair(
        digit1.map(|s: &str| s.parse().unwrap()),
        tag(" "),
        bag
    )(input)
}

fn bag_list(input: &str) -> IResult<&str, Option<Vec<(u32, &str)>>> {
    alt((
        tag("no other bags").map(|_| None),
        separated_list1(tag(", "), bag_count).map(|v| Some(v))
    ))(input)
}

fn bag_spec(input: &str) -> (&str, Option<Vec<(u32, &str)>>) {
    let (_rest, (bag, bag_spec)) = all_consuming(terminated(
        separated_pair(bag, tag(" contain "), bag_list),
        tag("."),
    ))(input).ok().unwrap();
    (bag, bag_spec)
}