use nom::AsBytes;

pub fn parse(input: &str) -> Vec<&str> {
    input
        .lines()
        // .map(|line| u16::from_str_radix(line, 2).unwrap())
        .collect()
}

pub fn part1(nums: &[&str]) -> usize {
    let num_digits = nums[0].len();
    let mut gamma = 0usize;
    let mut epsilon = 0usize;
    for i in 1..=num_digits {
        let count = nums.iter().fold(0, |count, line| {
            if line.as_bytes()[i - 1] == b'1' {
                count + 1
            } else {
                count - 1
            }
        });
        dbg!(count);
        let digit = if count >= 0 { 1 } else { 0 };
        let digit2 = if digit == 1 { 0 } else { 1 };
        gamma |= digit << (num_digits - i);
        epsilon |= digit2 << (num_digits - i);
    }

    // println!("{gamma:b} {epsilon:b}");
    gamma * epsilon
}

pub fn part2(nums: &[&str]) -> usize {
    let num_digits = nums[0].len();
    let mut oxygen_nums = nums.to_vec();
    let mut co2_nums = nums.to_vec();

    for pos in (0..num_digits) {
        let most_common = most_common_in_pos(pos, &oxygen_nums);
        dbg!(oxygen_nums.len());
        oxygen_nums.retain(|num| num.as_bytes()[pos] - b'0' == most_common);
        if oxygen_nums.len() == 1 {
            break;
        }
    }

    for pos in (0..num_digits) {
        let least_common = flip(most_common_in_pos(pos, &co2_nums));
        co2_nums.retain(|num| num.as_bytes()[pos] - b'0' == least_common);
        if co2_nums.len() == 1 {
            break;
        }
    }

    dbg!(&oxygen_nums);
    dbg!(&co2_nums);
    let ox_rating = usize::from_str_radix(oxygen_nums[0], 2).unwrap();
    let co2_rating = usize::from_str_radix(co2_nums[0], 2).unwrap();

    ox_rating * co2_rating
}

fn flip(digit: u8) -> u8 {
    match digit {
        1 => 0,
        0 => 1,
        _ => panic!(digit),
    }
}

fn most_common_in_pos(pos: usize, nums: &[&str]) -> u8 {
    let count = nums.iter().fold(0, |count, line| {
        if line.as_bytes()[pos] == b'1' {
            count + 1
        } else {
            count - 1
        }
    });
    if dbg!(count) >= 0 {
        1
    } else {
        0
    }
}
