use advent::prelude::*;

#[derive(HasParser)]
#[parse(sep_by = ",")]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Byte,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, EnumIter)]
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

fn shortest_path(map: &Grid<Cell>) -> usize {
    let mut to_explore = vec![(0, 0)];
    let mut counts = Grid::new(vec![vec![usize::MAX; map.width()]; map.height()]).unwrap();
    counts[0][0] = 0;
    while let Some((y, x)) = to_explore.pop() {
        let my_count = counts[y][x];
        for d in Direction::iter() {
            if let Some((ny, nx)) = d.advance(y, x, map.width(), map.height()) {
                if map[ny][nx] == Cell::Byte {
                    continue;
                }

                let new_score = my_count + 1;
                if new_score < counts[ny][nx] {
                    counts[ny][nx] = new_score;
                    to_explore.push((ny, nx));
                }
            }
        }
    }
    counts[map.height() - 1][map.width() - 1]
}

#[part_one]
fn part_one(bytes: List<Point, TermWith<NewLine>>) -> usize {
    let mut map = Grid::new(vec![vec![Cell::Empty; 71]; 71]).unwrap();
    for b in bytes.iter().take(1024) {
        map[b.y][b.x] = Cell::Byte;
    }

    shortest_path(&map)
}

#[part_two]
fn part_two(bytes: List<Point, TermWith<NewLine>>) -> String {
    let mut map = Grid::new(vec![vec![Cell::Empty; 71]; 71]).unwrap();
    for b in bytes {
        map[b.y][b.x] = Cell::Byte;
        if shortest_path(&map) == usize::MAX {
            return format!("{},{}", b.x, b.y);
        }
    }

    panic!("not found");
}

harness!(part_1: 408, part_2: "45,16");
