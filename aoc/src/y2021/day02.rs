#[derive(Clone, Copy)]
pub enum Dir {
    Forward,
    Down,
    Up,
}

pub fn parse(input: &str) -> Vec<(Dir, i32)> {
    input
        .lines()
        .map(|line| {
            let (dir, n) = line.split_once(" ").unwrap();
            let n: i32 = str::parse(n).unwrap();
            match dir {
                "forward" => (Dir::Forward, n),
                "down" => (Dir::Down, n),
                "up" => (Dir::Up, n),
                _ => unreachable!(),
            }
        })
        .collect()
}

pub fn part1(lines: &[(Dir, i32)]) -> usize {
    let (depth, dist) = lines
        .iter()
        .fold((0i32, 0i32), |(depth, dist), &(op, n)| match op {
            Dir::Forward => (depth, dist + n),
            Dir::Down => (depth + n, dist),
            Dir::Up => (depth - n, dist),
        });
    (depth * dist) as usize
}

pub fn part2(lines: &[(Dir, i32)]) -> usize {
    let (depth, dist, aim) =
        lines
            .iter()
            .fold((0, 0, 0), |(depth, dist, aim), &(dir, n)| match dir {
                Dir::Forward => (depth + (aim * n), dist + n, aim),
                Dir::Down => (depth, dist, aim + n),
                Dir::Up => (depth, dist, aim - n),
            });

    (depth * dist) as usize
}
