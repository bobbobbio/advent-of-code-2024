use advent::prelude::*;

#[derive(Debug, HasParser)]
#[parse(sep_by = "   ")]
struct InputElement {
    a: i32,
    b: i32
}

#[part_one]
fn part_one(input: List<InputElement, TermWith<NewLine>>) -> i32 {
    let mut list_1: Vec<_> = input.iter().map(|e| e.a).collect();
    let mut list_2: Vec<_> = input.iter().map(|e| e.b).collect();

    list_1.sort();
    list_2.sort();

    list_1.into_iter().zip(list_2.into_iter()).map(|(a, b)| (a - b).abs()).sum()
}

#[part_two]
fn part_two(input: List<InputElement, TermWith<NewLine>>) -> i32 {
    let list_1: Vec<_> = input.iter().map(|e| e.a).collect();
    let mut list_2 = HashMap::new();
    for n in input.iter().map(|e| e.b) {
        *list_2.entry(n).or_default() += 1;
    }

    list_1.iter().map(|e| e * list_2.get(e).unwrap_or(&0)).sum()
}

harness!();
