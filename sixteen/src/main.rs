use advent::prelude::*;

#[derive(Copy, Clone, Debug, DisplayMore, HasParser, PartialEq, Eq)]
enum Cell {
    #[display("#")]
    #[parse(string = "#")]
    Wall,
    #[display(".")]
    #[parse(string = ".")]
    Empty,
    #[display("S")]
    #[parse(string = "S")]
    Start,
    #[display("E")]
    #[parse(string = "E")]
    End,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, EnumIter)]
#[repr(usize)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    fn advance(&self, y: usize, x: usize, width: usize, height: usize) -> Option<(usize, usize)> {
        match self {
            Self::Up => (y > 0).then(|| (y - 1, x)),
            Self::Down => (y < height - 1).then(|| (y + 1, x)),
            Self::Left => (x > 0).then(|| (y, x - 1)),
            Self::Right => (x < width - 1).then(|| (y, x + 1)),
        }
    }
}

fn min_path(y: usize, x: usize, d: Direction, map: &Grid<Cell>) -> usize {
    let mut costs = Grid::new(vec![vec![usize::MAX; map.width()]; map.height()]).unwrap();
    costs[y][x] = 0;

    let mut to_explore = vec![(y, x, d)];

    while let Some((y, x, d)) = to_explore.pop() {
        let score = costs[y][x];

        for new_d in Direction::iter() {
            if let Some((ny, nx)) = new_d.advance(y, x, map.width(), map.height()) {
                if map[ny][nx] == Cell::Wall {
                    continue;
                }

                let new_score = if new_d == d { score + 1 } else { score + 1001 };

                if new_score < costs[ny][nx] {
                    costs[ny][nx] = new_score;
                    to_explore.push((ny, nx, new_d));
                }
            }
        }
    }

    let (y, x) = map.position(|&c| c == Cell::End).unwrap();
    costs[y][x]
}

#[part_one]
fn part_one(map: Grid<Cell>) -> usize {
    let (start_y, start_x) = map.position(|&c| c == Cell::Start).unwrap();
    min_path(start_y, start_x, Direction::Right, &map)
}

#[derive(Clone, Debug)]
struct CostEntry {
    cost: usize,
    positions: Vec<(usize, usize, Direction)>,
}

impl CostEntry {
    fn new() -> Self {
        Self {
            cost: usize::MAX,
            positions: vec![],
        }
    }
}

fn min_path2(start_y: usize, start_x: usize, d: Direction, map: &Grid<Cell>) -> usize {
    let (end_y, end_x) = map.position(|&c| c == Cell::End).unwrap();

    let mut costs =
        vec![Grid::new(vec![vec![CostEntry::new(); map.width()]; map.height()]).unwrap(); 4];
    for d in Direction::iter() {
        costs[d as usize][start_y][start_x].cost = 0;
    }

    let mut to_explore = vec![(start_y, start_x, d)];

    while let Some((y, x, d)) = to_explore.pop() {
        if (y, x) == (end_y, end_x) {
            continue;
        }

        let score = costs[d as usize][y][x].cost;

        for new_d in Direction::iter() {
            if let Some((ny, nx)) = new_d.advance(y, x, map.width(), map.height()) {
                if map[ny][nx] == Cell::Wall {
                    continue;
                }

                let new_score = if new_d == d { score + 1 } else { score + 1001 };

                match new_score.cmp(&costs[new_d as usize][ny][nx].cost) {
                    std::cmp::Ordering::Less => {
                        costs[new_d as usize][ny][nx] = CostEntry {
                            cost: new_score,
                            positions: vec![(y, x, d)],
                        };
                        to_explore.push((ny, nx, new_d));
                    }
                    std::cmp::Ordering::Equal => {
                        costs[new_d as usize][ny][nx].positions.push((y, x, d));
                        to_explore.push((ny, nx, new_d));
                    }
                    _ => (),
                }
            }
        }
    }

    let min_cost = Direction::iter()
        .map(|d| costs[d as usize][end_y][end_x].cost)
        .min()
        .unwrap();
    let mut positions = HashSet::new();
    for d in Direction::iter() {
        if costs[d as usize][end_y][end_x].cost > min_cost {
            continue;
        }
        let mut to_explore = vec![(end_y, end_x, d)];
        let mut visited = HashSet::new();
        while let Some((y, x, d)) = to_explore.pop() {
            if visited.contains(&(y, x, d)) {
                continue;
            }
            positions.insert((y, x));
            to_explore.extend(costs[d as usize][y][x].positions.iter().cloned());
            visited.insert((y, x, d));
        }
    }

    positions.len()
}

#[part_two]
fn part_two(map: Grid<Cell>) -> usize {
    let (start_y, start_x) = map.position(|&c| c == Cell::Start).unwrap();
    min_path2(start_y, start_x, Direction::Right, &map)
}

harness!(part_1: 90460, part_2: 575);
