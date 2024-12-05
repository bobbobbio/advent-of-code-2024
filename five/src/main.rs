use advent::prelude::*;

#[derive(Debug, HasParser)]
#[parse(sep_by = "|")]
struct PageNumberPair(u32, u32);

#[derive(Debug, HasParser)]
#[parse(sep_by = "\n")]
struct Input {
    ordering_rules: List<PageNumberPair, TermWith<NewLine>>,
    page_numbers: List<List<u32, SepBy<Comma>>, TermWith<NewLine>>,
}

fn is_valid(numbers: &[u32], rules: &[PageNumberPair]) -> bool {
    let index_map: HashMap<_, _> = numbers.iter().enumerate().map(|(i, n)| (n, i)).collect();
    for rule in rules {
        if !index_map.contains_key(&rule.0) || !index_map.contains_key(&rule.1) {
            continue;
        }
        if index_map[&rule.0] > index_map[&rule.1] {
            return false;
        }
    }
    true
}

#[part_one]
fn part_one(input: Input) -> u32 {
    let mut total = 0;
    for numbers in &input.page_numbers {
        if is_valid(&numbers[..], &input.ordering_rules[..]) {
            total += numbers[numbers.len() / 2];
        }
    }
    total
}

fn reorder_page_numbers(numbers: &mut [u32], rules: &[PageNumberPair]) -> bool {
    let mut did_swap = false;
    let mut index_map: HashMap<u32, usize> =
        numbers.iter().enumerate().map(|(i, n)| (*n, i)).collect();
    loop {
        let mut swaps = 0;
        for rule in rules {
            if !index_map.contains_key(&rule.0) || !index_map.contains_key(&rule.1) {
                continue;
            }
            let index_a = index_map[&rule.0];
            let index_b = index_map[&rule.1];
            if index_a > index_b {
                numbers.swap(index_a, index_b);
                index_map.insert(rule.0, index_b);
                index_map.insert(rule.1, index_a);
                swaps += 1;
                did_swap = true;
            }
        }
        if swaps == 0 {
            break;
        }
    }
    did_swap
}

#[part_two]
fn part_two(input: Input) -> u32 {
    let mut total = 0;
    for numbers in &input.page_numbers {
        let mut numbers = numbers.to_vec();
        if reorder_page_numbers(&mut numbers, &input.ordering_rules[..]) {
            total += numbers[numbers.len() / 2];
        }
    }
    total
}

harness!(part_1: 5762, part_2: 4130);
