use advent::prelude::*;

fn next_stones(s: u128, v: usize, add_to: &mut HashMap<u128, usize>) {
    if s == 0 {
        *add_to.entry(1).or_default() += v;
    } else {
        let num_digits = ((s + 1) as f64).log(10.0).ceil();
        if num_digits % 2.0 == 0.0 {
            *add_to
                .entry(s / 10u128.pow(num_digits as u32 / 2))
                .or_default() += v;
            *add_to
                .entry(s % 10u128.pow(num_digits as u32 / 2))
                .or_default() += v;
        } else {
            *add_to.entry(s * 2024).or_default() += v;
        }
    }
}

fn solve(input: List<u128, SepBy<Space>>, times: usize) -> usize {
    let mut stones = HashMap::new();
    for i in input {
        next_stones(i, 1, &mut stones);
    }

    for _ in 0..(times - 1) {
        let mut new_stones = HashMap::new();
        for (&k, &v) in &stones {
            next_stones(k, v, &mut new_stones);
        }

        stones = new_stones;
    }

    stones.values().copied().sum()
}

#[part_one]
fn part_one(input: List<u128, SepBy<Space>>) -> usize {
    solve(input, 25)
}

#[part_two]
fn part_two(input: List<u128, SepBy<Space>>) -> usize {
    solve(input, 75)
}

harness!(part_1: 190865, part_2: 225404711855335);
