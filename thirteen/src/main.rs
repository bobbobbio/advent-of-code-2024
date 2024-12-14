use advent::prelude::*;

#[derive(Clone, Debug, HasParser, PartialEq, Eq, Hash)]
#[parse(sep_by = ", ")]
struct Delta {
    #[parse(before = "X+")]
    x: i64,
    #[parse(before = "Y+")]
    y: i64,
}

#[derive(Clone, Debug, HasParser, PartialEq, Eq, Hash)]
#[parse(sep_by = ", ")]
struct Prize {
    #[parse(before = "X=")]
    x: i64,
    #[parse(before = "Y=")]
    y: i64,
}

#[derive(Clone, Debug, HasParser, PartialEq, Eq, Hash)]
#[parse(sep_by = "\n")]
struct Machine {
    #[parse(before = "Button A: ")]
    a: Delta,
    #[parse(before = "Button B: ")]
    b: Delta,
    #[parse(before = "Prize: ", after = "\n")]
    prize: Prize,
}

/*
 * c1 * a_x + c2 * b_x = p_x
 * c1 * a_y + c2 * b_y = p_y
 *
 * c2 * b_y = p_y - c1 * a_y
 * c2 = (p_y - c1 * a_y) / b_y
 *
 * c2 * b_x = p_x - c1 * a_x
 * c2 = (p_x - c1 * a_x) / b_x
 *
 * (p_y - c1 * a_y) / b_y = (p_x - c1 * a_x) / b_x
 * p_y / b_y - (c1 * a_y) / b_y = p_x / b_x - (c1 * a_x) / b_x
 * (c1 * a_x) / b_x - (c1 * a_y) / b_y = p_x / b_x - p_y / b_y
 * c1(a_x / b_x - a_y / b_y) = p_x / b_x - p_y / b_y
 * c1 = (p_x / b_x - p_y / b_y) / (a_x / b_x - a_y / b_y)
 * c1 = (p_x / b_x - p_y / b_y) / (a_x * b_y / (b_x * b_y) - (a_y * b_x) / (b_y * b_x))
 * c1 = (p_x / b_x - p_y / b_y) / (a_x * b_y - a_y * b_x) / (b_y * b_x)
 * c1 = (p_x / b_x - p_y / b_y) * (b_y * b_x) / (a_x * b_y - a_y * b_x)
 * c1 = (p_x * b_y - p_y * b_x) / (a_x * b_y - a_y * b_x)
 *
 */

fn minimum_cost(m: Machine) -> Option<i64> {
    let n = m.prize.x * m.b.y - m.prize.y * m.b.x;
    let d = m.a.x * m.b.y - m.a.y * m.b.x;
    if n % d == 0 {
        let a = n / d;

        let n = m.prize.x - a * m.a.x;
        let d = m.b.x;
        if n % d == 0 {
            let b = n / d;
            Some(a * 3 + b)
        } else {
            None
        }
    } else {
        None
    }
}

#[part_one]
fn part_one(input: List<Machine, SepBy<NewLine>>) -> i64 {
    input.into_iter().map(|m| minimum_cost(m)).flatten().sum()
}

#[part_two]
fn part_two(input: List<Machine, SepBy<NewLine>>) -> i64 {
    input
        .into_iter()
        .map(|mut m| {
            m.prize.x += 10000000000000;
            m.prize.y += 10000000000000;
            minimum_cost(m)
        })
        .flatten()
        .sum()
}

harness!(part_1: 28059, part_2: 102255878088512);
