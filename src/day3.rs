use ndarray::Array2;

pub fn parse(input: &str) -> Array2<bool> {
    let cols = input.lines().next().map(|line| line.len()).expect("No lines");
    let grid_flat: Vec<_> = input
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| {
                match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!("Unexpected char {}", c),
                }
            })
        })
        .collect();

    let rows = grid_flat.len() / cols;
    Array2::from_shape_vec((rows, cols), grid_flat).expect("Can't build array")
}

fn stride_for_slope(cols: usize, right: usize, down: usize) -> usize {
    right + (down * cols)
}

pub fn part1(input: &Array2<bool>) -> u32 {
    check_slope(input, (3, 1))
}

pub fn part2(input: &Array2<bool>) -> u32 {
    [
        (1, 1),
        (3, 1),
        (5, 1), 
        (7, 1),
        (1, 2),
    ]
    .iter()
    .map(|slope| check_slope(input, *slope))
    .product()
}

fn check_slope(input: &Array2<bool>, (right, down): (usize, usize)) -> u32 {
    let mut col = 0;
    let mut tree_count = 0;
    for row in (0..input.nrows()).step_by(down) {
        if input[(row, col)] {
            tree_count += 1;
        }
        col = (col + right) % input.ncols();
    }
    tree_count
}

pub fn run() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("input/2020/day3.txt")?;
    let parsed = parse(&contents);
    println!("{}", part1(&parsed));
    println!("{}", part2(&parsed));
    Ok(())
}