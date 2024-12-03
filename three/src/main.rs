use advent::prelude::*;

#[derive(Debug, HasParser)]
enum Action {
    #[parse(before = "mul(", after = ")", sep_by = ",")]
    Mul(u32, u32),
    #[parse(string = "do()")]
    Do,
    #[parse(string = "don't()")]
    Dont,
}

#[derive(Debug)]
struct MulList(Vec<Action>);

impl HasParser for MulList {
    #[into_parser]
    fn parser() -> _ {
        many1(attempt(Action::parser().map(|v| Some(v))).or(any().map(|_| None)))
            .map(|m: Vec<_>| Self(m.into_iter().flatten().collect()))
    }
}

#[part_one]
fn part_one(m: MulList) -> u32 {
    m.0.into_iter()
        .filter_map(|a| match a {
            Action::Mul(a, b) => Some(a * b),
            _ => None,
        })
        .sum()
}

#[part_two]
fn part_two(m: MulList) -> u32 {
    let mut total = 0;
    let mut enabled = true;
    for e in m.0.into_iter() {
        match e {
            Action::Mul(a, b) => {
                if enabled {
                    total += a * b;
                }
            }
            Action::Do => {
                enabled = true;
            }
            Action::Dont => {
                enabled = false;
            }
        }
    }
    total
}

harness!(part_1: 173529487, part_2: 99532691);
