use advent::prelude::*;

fn is_safe(line: &[i32]) -> bool {
    let diffs: Vec<_> = line
        .iter()
        .zip(line.iter().skip(1))
        .map(|(a, b)| a - b)
        .collect();
    diffs.iter().all(|&d| d > 0 && d <= 3) || diffs.iter().all(|&d| (-3..0).contains(&d))
}

#[part_one]
fn part_one(input: List<List<i32, SepBy<Space>>, TermWith<NewLine>>) -> i32 {
    let mut safe = 0;
    for line in input {
        if is_safe(&line) {
            safe += 1;
        }
    }
    safe
}

#[part_two]
fn part_two(input: List<List<i32, SepBy<Space>>, TermWith<NewLine>>) -> i32 {
    let mut safe = 0;
    for line in input {
        if is_safe(&line) {
            safe += 1;
        } else {
            for r in 0..line.len() {
                let mut new_line = line.to_vec();
                new_line.remove(r);
                if is_safe(&new_line) {
                    safe += 1;
                    break;
                }
            }
        }
    }
    safe
}

harness!(part_1: 486, part_2: 540);
