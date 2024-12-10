use advent::prelude::*;

struct Digit(u64);

impl HasParser for Digit {
    #[into_parser]
    fn parser() -> _ {
        digit().map(|d| Self(d.to_string().parse().unwrap()))
    }
}

#[derive(EnumIter)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn advance(&self, y: usize, x: usize, width: usize, height: usize) -> Option<(usize, usize)> {
        match self {
            Self::North => (y > 0).then(|| (y - 1, x)),
            Self::South => (y < height - 1).then(|| (y + 1, x)),
            Self::West => (x > 0).then(|| (y, x - 1)),
            Self::East => (x < width - 1).then(|| (y, x + 1)),
        }
    }
}

fn find_trail_ends(
    input: &Grid<Digit>,
    trail_ends: &mut HashSet<(usize, usize)>,
    y: usize,
    x: usize,
) {
    if input[y][x].0 == 9 {
        trail_ends.insert((y, x));
        return;
    }

    for d in Direction::iter() {
        if let Some((ny, nx)) = d.advance(y, x, input.width(), input.height()) {
            if input[ny][nx].0 != input[y][x].0 + 1 {
                continue;
            }

            find_trail_ends(input, trail_ends, ny, nx);
        }
    }
}

#[part_one]
fn part_one(input: Grid<Digit>) -> u64 {
    let mut total = 0;
    for (y, x) in input.positions() {
        if input[y][x].0 == 0 {
            let mut trail_ends = HashSet::new();
            find_trail_ends(&input, &mut trail_ends, y, x);
            total += trail_ends.len() as u64;
        }
    }
    total
}

fn find_trails(input: &Grid<Digit>, y: usize, x: usize) -> u64 {
    if input[y][x].0 == 9 {
        return 1;
    }

    let mut total = 0;
    for d in Direction::iter() {
        if let Some((ny, nx)) = d.advance(y, x, input.width(), input.height()) {
            if input[ny][nx].0 != input[y][x].0 + 1 {
                continue;
            }

            total += find_trails(input, ny, nx);
        }
    }
    total
}

#[part_two]
fn part_two(input: Grid<Digit>) -> u64 {
    let mut total = 0;
    for (y, x) in input.positions() {
        if input[y][x].0 == 0 {
            total += find_trails(&input, y, x);
        }
    }
    total
}

harness!(part_1: 548, part_2: 1252);
