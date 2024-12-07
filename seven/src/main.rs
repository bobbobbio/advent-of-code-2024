use advent::prelude::*;

#[derive(Clone, HasParser)]
#[parse(sep_by = ": ")]
struct InputLine {
    test_value: u64,
    input: List<u64, SepBy<Space>>,
}

enum Operator {
    Mult,
    Add,
    Concat,
}

impl Operator {
    fn apply_reverse(&self, a: u64, b: u64) -> Option<u64> {
        match self {
            Self::Mult => (a % b == 0).then(|| a / b),
            Self::Add => (a >= b).then(|| a - b),
            Self::Concat => (a != b)
                .then(|| {
                    a.to_string()
                        .strip_suffix(&b.to_string())
                        .map(|p| p.parse().unwrap())
                })
                .flatten(),
        }
    }
}

fn is_valid(mut line: InputLine, operators: &[Operator]) -> bool {
    if line.input.len() == 1 {
        line.input[0] == line.test_value
    } else {
        let top = line.input.pop().unwrap();

        for op in operators {
            let mut new_line = line.clone();
            if let Some(value) = op.apply_reverse(new_line.test_value, top) {
                new_line.test_value = value;
                if is_valid(new_line, operators) {
                    return true;
                }
            }
        }

        false
    }
}

fn solve(lines: List<InputLine, TermWith<NewLine>>, operators: &[Operator]) -> u64 {
    let mut total = 0;
    for line in lines {
        let test_value = line.test_value;
        if is_valid(line, operators) {
            total += test_value;
        }
    }
    total
}

#[part_one]
fn part_one(lines: List<InputLine, TermWith<NewLine>>) -> u64 {
    solve(lines, &[Operator::Mult, Operator::Add])
}

#[part_two]
fn part_two(lines: List<InputLine, TermWith<NewLine>>) -> u64 {
    solve(lines, &[Operator::Mult, Operator::Add, Operator::Concat])
}

harness!(part_1: 1985268524462, part_2: 150077710195188);
