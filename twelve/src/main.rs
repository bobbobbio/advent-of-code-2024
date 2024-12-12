use advent::prelude::*;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, EnumIter)]
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

fn calculate_perimeter(
    iter: impl Iterator<Item = (usize, usize)>,
    width: usize,
    height: usize,
) -> usize {
    let mut perimeter = 0;
    let positions = HashSet::<(usize, usize)>::from_iter(iter);
    for &(y, x) in &positions {
        for d in Direction::iter() {
            if let Some((ny, nx)) = d.advance(y, x, width, height) {
                if !positions.contains(&(ny, nx)) {
                    perimeter += 1;
                }
            } else {
                perimeter += 1;
            }
        }
    }
    perimeter
}

fn calculate_groups(input: Grid<char>) -> (Grid<usize>, usize) {
    let mut groups = Grid::new(vec![vec![0; input.width()]; input.height()]).unwrap();
    let mut next_group = 1;

    let mut group_connections = vec![];
    for (y, x) in input.positions() {
        let mut neighbor_groups = vec![];
        for d in [Direction::North, Direction::West] {
            if let Some((ny, nx)) = d.advance(y, x, input.width(), input.height()) {
                if input[ny][nx] != input[y][x] {
                    continue;
                }
                if groups[ny][nx] > 0 {
                    neighbor_groups.push(groups[ny][nx]);
                }
            }
        }
        if neighbor_groups.is_empty() {
            groups[y][x] = next_group;
            next_group += 1;
        } else {
            groups[y][x] = neighbor_groups[0];
            if neighbor_groups.len() > 1 {
                assert!(neighbor_groups.len() == 2);
                group_connections.push((neighbor_groups[0], neighbor_groups[1]));
            }
        }
    }
    while let Some((a, b)) = group_connections.pop() {
        for c in groups.cells_mut() {
            if *c == a {
                *c = b;
            }
        }
        for (ao, bo) in &mut group_connections {
            if *ao == a {
                *ao = b;
            }
            if *bo == a {
                *bo = b;
            }
        }
    }
    (groups, next_group)
}

#[part_one]
fn part_one(input: Grid<char>) -> usize {
    let (groups, next_group) = calculate_groups(input);

    let mut total = 0;
    for g in 1..next_group {
        let iter = || groups.positions().filter(|&(y, x)| groups[y][x] == g);
        let area = iter().count();
        if area == 0 {
            continue;
        }
        let perimeter = calculate_perimeter(iter(), groups.width(), groups.height());

        total += area * perimeter;
    }
    total
}

fn calculate_sides(
    iter: impl Iterator<Item = (usize, usize)>,
    width: usize,
    height: usize,
) -> usize {
    let positions = BTreeSet::<(usize, usize)>::from_iter(iter);

    let mut next_group = 0;
    let mut fences = Grid::new(vec![
        vec![HashMap::<Direction, usize>::new(); width];
        height
    ])
    .unwrap();
    for &(y, x) in &positions {
        let mut my_fences = vec![];
        for d in Direction::iter() {
            if let Some((ny, nx)) = d.advance(y, x, width, height) {
                if !positions.contains(&(ny, nx)) {
                    my_fences.push(d);
                }
            } else {
                my_fences.push(d);
            }
        }
        for fence in my_fences {
            let check = match &fence {
                Direction::North | Direction::South => Direction::West,
                Direction::East | Direction::West => Direction::North,
            };

            if let Some((ny, nx)) = check.advance(y, x, width, height) {
                if let Some(gn) = fences[ny][nx].get(&fence) {
                    let gn = *gn;
                    fences[y][x].insert(fence, gn);
                    continue;
                }
            }
            fences[y][x].insert(fence, next_group);
            next_group += 1;
        }
    }

    next_group
}

#[part_two]
fn part_two(input: Grid<char>) -> usize {
    let (groups, next_group) = calculate_groups(input);

    let mut total = 0;
    for g in 1..next_group {
        let iter = || groups.positions().filter(|&(y, x)| groups[y][x] == g);
        let area = iter().count();
        if area == 0 {
            continue;
        }
        let sides = calculate_sides(iter(), groups.width(), groups.height());

        total += area * sides;
    }
    total
}

harness!(part_1: 1381056, part_2: 834828);
