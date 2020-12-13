#![feature(str_split_once)]
#![feature(iterator_fold_self)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

macro_rules! match_day {
    ($day_var:ident, $contents:expr, [ $( $d:ident ),* $(,)?]) => {
        match format!("day{}", $day_var).as_str() {
            $(
                stringify!($d) => {
                    let parsed = $crate::$d::parse(&$contents);
                    let part1 = $crate::$d::part1(&parsed);
                    let part2 = $crate::$d::part2(&parsed);
                    println!("Part 1: {}", part1);
                    println!("Part 2: {}", part2);
                }
            )*
            _ => unimplemented!("Don't have day {} yet", $day_var)
        }
    };
}

fn main() -> anyhow::Result<()> {
    let day = 8;

    let contents = std::fs::read_to_string(format!("input/2020/day{}.txt", day))?;
    match_day!(day, contents, [
        day1,
        day2,
        day3,
        day4,
        day5,
        day6,
        day7,
        day8,
    ]);
    Ok(())
}
