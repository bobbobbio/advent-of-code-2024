use advent::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Antenna(char);

impl HasParser for Antenna {
    #[into_parser]
    fn parser() -> _ {
        letter().or(digit()).map(Self)
    }
}

#[derive(HasParser)]
enum InputCell {
    Antenna(Antenna),
    #[parse(string = ".")]
    Empty,
}

#[derive(Copy, Clone, DisplayMore, PartialEq, Eq)]
enum Cell {
    #[display("#")]
    AntiNode,
    #[display(".")]
    Empty,
}

fn third_point(a: usize, b: usize, bound: usize) -> Option<usize> {
    if a > b {
        let delta = a - b;
        (delta <= b).then(|| b - delta)
    } else {
        let delta = b - a;
        (b + delta < bound).then(|| b + delta)
    }
}

fn antinode(
    (ay, ax): (usize, usize),
    (by, bx): (usize, usize),
    (width, height): (usize, usize),
) -> Option<(usize, usize)> {
    Some((third_point(ay, by, height)?, third_point(ax, bx, width)?))
}

fn solve(
    input: Grid<InputCell>,
    mut loop_body: impl FnMut(&mut Grid<Cell>, (usize, usize), (usize, usize)),
) -> usize {
    let mut pos = HashMap::<Antenna, Vec<(usize, usize)>>::new();
    for (y, x) in input.positions() {
        if let InputCell::Antenna(a) = input[y][x] {
            let l = pos.entry(a).or_default();
            l.push((y, x));
        }
    }
    let mut overlay = Grid::new(vec![vec![Cell::Empty; input.width()]; input.height()]).unwrap();

    for locations in pos.values() {
        if locations.len() < 2 {
            continue;
        }
        for i in 0..locations.len() {
            let a = &locations[i];
            for b in &locations[i + 1..] {
                loop_body(&mut overlay, *a, *b);
            }
        }
    }
    overlay.cells().filter(|&&c| c == Cell::AntiNode).count()
}

#[part_one]
fn part_one(input: Grid<InputCell>) -> usize {
    solve(input, |overlay, a, b| {
        let mut f = |a, b| {
            if let Some(r) = antinode(a, b, (overlay.width(), overlay.height())) {
                overlay[r.0][r.1] = Cell::AntiNode;
            }
        };
        f(a, b);
        f(b, a);
    })
}

#[part_two]
fn part_two(input: Grid<InputCell>) -> usize {
    solve(input, |overlay, a, b| {
        overlay[a.0][a.1] = Cell::AntiNode;
        overlay[b.0][b.1] = Cell::AntiNode;

        let mut f = |mut n_a, mut n_b| {
            while let Some(r) = antinode(n_a, n_b, (overlay.width(), overlay.height())) {
                overlay[r.0][r.1] = Cell::AntiNode;
                n_a = n_b;
                n_b = r;
            }
        };
        f(a, b);
        f(b, a);
    })
}

harness!(part_1: 273, part_2: 1017);
