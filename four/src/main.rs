use advent::prelude::*;

fn find_str(
    input: &Grid<char>,
    mut x: usize,
    mut y: usize,
    direction: Direction8,
    s: &str,
) -> bool {
    let mut msg = String::new();

    for _ in 0..s.len() {
        msg.push(input[y][x]);
        let Some((nx, ny)) = direction.advance(x, y, input.width(), input.height()) else {
            break;
        };
        x = nx;
        y = ny;
    }

    msg == s
}

#[part_one]
fn part_one(input: Grid<char>) -> u32 {
    let mut total = 0;
    for (y, x) in input.positions() {
        for direction in Direction8::iter() {
            if find_str(&input, x, y, direction, "XMAS") {
                total += 1;
            }
        }
    }
    total
}

fn find_x_mas(input: &Grid<char>, x: usize, y: usize) -> bool {
    if x + 2 >= input.width() || y + 2 >= input.height() {
        return false;
    }

    (find_str(input, x, y, Direction8::SouthEast, "MAS")
        || find_str(input, x + 2, y + 2, Direction8::NorthWest, "MAS"))
        && (find_str(input, x + 2, y, Direction8::SouthWest, "MAS")
            || find_str(input, x, y + 2, Direction8::NorthEast, "MAS"))
}

#[part_two]
fn part_two(input: Grid<char>) -> u32 {
    let mut total = 0;
    for (y, x) in input.positions() {
        if find_x_mas(&input, x, y) {
            total += 1;
        }
    }
    total
}

harness!(part_1: 2562, part_2: 1902);
