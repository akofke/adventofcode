
pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &[&str]) -> usize {
    input
        .iter()
        .map(|line| {
            let (row, col) = get_seat(line);
            let seat_id = row * 8 + col;
            seat_id
        })
        .max().unwrap()
}

pub fn part2(input: &[&str]) -> usize {
    let mut seats: Vec<_> = input
        .iter()
        .map(|line| {
            let (row, col) = get_seat(line);
            let seat_id = row * 8 + col;
            seat_id
        })
        .collect();
    seats.sort();
    let window = seats
        .windows(2)
        .find(|window| {
            window[1] - window[0] == 2
        }).unwrap();
    window[0] + 1
}

fn get_seat(line: &str) -> (usize, usize) {
    let (row_str, col_str) = line.split_at(7);
    let row = row_str
        .chars()
        .fold((0, 128), |(lo, hi), c| {
            let mid = lo + ((lo..hi).len() / 2);
            match c {
                'F' => (lo, mid),
                'B' => (mid, hi),
                _ => unreachable!(),
            }
        })
        .0;

    let col = col_str
        .chars()
        .fold((0, 8), |(lo, hi), c| {
            let mid = lo + ((lo..hi).len() / 2);
            match c {
                'L' => (lo, mid),
                'R' => (mid, hi),
                _ => unreachable!(),
            }
        })
        .0;
    (row, col)
}