use advent::prelude::*;
use strum::IntoEnumIterator as _;
use strum_macros::EnumIter;

#[derive(EnumIter)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    fn advance(&self, x: usize, y: usize, width: usize, height: usize) -> Option<(isize, isize)> {
        let mut dx = 0;
        let mut dy = 0;
        match self {
            Self::North | Self::NorthEast | Self::NorthWest => {
                if y == 0 {
                    return None;
                }
                dy -= 1;
            }
            Self::South | Self::SouthEast | Self::SouthWest => {
                if y >= height - 1 {
                    return None;
                }
                dy += 1;
            }
            _ => {}
        }
        match self {
            Self::East | Self::NorthEast | Self::SouthEast => {
                if x >= width - 1 {
                    return None;
                }
                dx += 1;
            }
            Self::West | Self::NorthWest | Self::SouthWest => {
                if x == 0 {
                    return None;
                }
                dx -= 1;
            }
            _ => {}
        }
        Some((dx, dy))
    }
}

fn find_str(input: &Grid<char>, mut x: usize, mut y: usize, direction: Direction, s: &str) -> bool {
    let mut msg = String::new();

    for _ in 0..s.len() {
        msg.push(input[y][x]);
        let Some((dx, dy)) = direction.advance(x, y, input.width(), input.height()) else {
            break;
        };
        x = (x as isize + dx) as usize;
        y = (y as isize + dy) as usize;
    }

    msg == s
}

#[part_one]
fn part_one(input: Grid<char>) -> u32 {
    let mut total = 0;
    for x in 0..input.width() {
        for y in 0..input.height() {
            for direction in Direction::iter() {
                if find_str(&input, x, y, direction, "XMAS") {
                    total += 1;
                }
            }
        }
    }
    total
}

fn find_x_mas(input: &Grid<char>, x: usize, y: usize) -> bool {
    if x + 2 >= input.width() || y + 2 >= input.height() {
        return false;
    }

    (find_str(input, x, y, Direction::SouthEast, "MAS")
        || find_str(input, x + 2, y + 2, Direction::NorthWest, "MAS"))
        && (find_str(input, x + 2, y, Direction::SouthWest, "MAS")
            || find_str(input, x, y + 2, Direction::NorthEast, "MAS"))
}

#[part_two]
fn part_two(input: Grid<char>) -> u32 {
    let mut total = 0;
    for x in 0..input.width() {
        for y in 0..input.height() {
            if find_x_mas(&input, x, y) {
                total += 1;
            }
        }
    }
    total
}

harness!(part_1: 2562, part_2: 1902);
