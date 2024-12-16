use advent::prelude::*;

#[derive(Copy, Clone, Debug, DisplayMore, HasParser, PartialEq, Eq)]
enum Cell {
    #[display("#")]
    #[parse(string = "#")]
    Wall,
    #[display("O")]
    #[parse(string = "O")]
    Box,
    #[display(".")]
    #[parse(string = ".")]
    Empty,
    #[display("@")]
    #[parse(string = "@")]
    Robot,
}

#[derive(Copy, Clone, Debug, HasParser, PartialEq, Eq)]
enum Direction {
    #[parse(string = "^")]
    Up,
    #[parse(string = "v")]
    Down,
    #[parse(string = "<")]
    Left,
    #[parse(string = ">")]
    Right,
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

#[derive(HasParser)]
#[parse(sep_by = "\n")]
struct Input {
    map: Grid<Cell>,
    instructions: List<Vec<Direction>, TermWith<NewLine>>,
}

fn attempt_move(
    y: usize,
    x: usize,
    direction: Direction,
    map: &mut Grid<Cell>,
) -> Option<(usize, usize)> {
    let (ny, nx) = direction.advance(y, x, map.width(), map.height())?;
    if map[ny][nx] == Cell::Wall {
        return None;
    }
    if map[ny][nx] == Cell::Box && attempt_move(ny, nx, direction, map).is_none() {
        return None;
    }

    assert_eq!(map[ny][nx], Cell::Empty);
    map[ny][nx] = map[y][x];
    map[y][x] = Cell::Empty;

    Some((ny, nx))
}

#[part_one]
fn part_one(mut input: Input) -> usize {
    let (mut robot_y, mut robot_x) = input.map.position(|&c| c == Cell::Robot).unwrap();

    for d in input.instructions.iter().flatten() {
        if let Some((ny, nx)) = attempt_move(robot_y, robot_x, *d, &mut input.map) {
            robot_y = ny;
            robot_x = nx;
        }
    }

    input
        .map
        .positions()
        .filter(|&(y, x)| input.map[y][x] == Cell::Box)
        .map(|(y, x)| y * 100 + x)
        .sum()
}

#[derive(Copy, Clone, Debug, DisplayMore, PartialEq, Eq)]
enum Cell2 {
    #[display("#")]
    Wall,
    #[display("[")]
    Box1,
    #[display("]")]
    Box2,
    #[display(".")]
    Empty,
    #[display("@")]
    Robot,
}

fn map2_from_map2(map: Grid<Cell>) -> Grid<Cell2> {
    let mut rows = vec![];
    for row in map.rows() {
        let mut new_row = vec![];
        for c in row.iter() {
            match c {
                Cell::Wall => {
                    new_row.push(Cell2::Wall);
                    new_row.push(Cell2::Wall);
                }
                Cell::Box => {
                    new_row.push(Cell2::Box1);
                    new_row.push(Cell2::Box2);
                }
                Cell::Empty => {
                    new_row.push(Cell2::Empty);
                    new_row.push(Cell2::Empty);
                }
                Cell::Robot => {
                    new_row.push(Cell2::Robot);
                    new_row.push(Cell2::Empty);
                }
            }
        }
        rows.push(new_row);
    }
    Grid::new(rows).unwrap()
}

fn attempt_move2(
    y: usize,
    x: usize,
    direction: Direction,
    dependent_move: bool,
    map: &mut Grid<Cell2>,
) -> Option<(usize, usize)> {
    let (ny, nx) = direction.advance(y, x, map.width(), map.height())?;
    if map[ny][nx] == Cell2::Wall {
        return None;
    }
    if (map[ny][nx] == Cell2::Box1 || map[ny][nx] == Cell2::Box2)
        && attempt_move2(ny, nx, direction, false, map).is_none()
    {
        return None;
    }

    if !dependent_move && (direction == Direction::Up || direction == Direction::Down) {
        if map[y][x] == Cell2::Box1 && attempt_move2(y, x + 1, direction, true, map).is_none() {
            return None;
        }
        if map[y][x] == Cell2::Box2 && attempt_move2(y, x - 1, direction, true, map).is_none() {
            return None;
        }
    }

    assert_eq!(map[ny][nx], Cell2::Empty);
    map[ny][nx] = map[y][x];
    map[y][x] = Cell2::Empty;

    Some((ny, nx))
}

#[part_two]
fn part_two(input: Input) -> usize {
    let mut map = map2_from_map2(input.map);

    let (mut robot_y, mut robot_x) = map.position(|&c| c == Cell2::Robot).unwrap();

    for d in input.instructions.iter().flatten() {
        let mut map2 = map.clone();
        if let Some((ny, nx)) = attempt_move2(robot_y, robot_x, *d, false, &mut map2) {
            robot_y = ny;
            robot_x = nx;
            map = map2;
        }
    }

    map.positions()
        .filter(|&(y, x)| map[y][x] == Cell2::Box1)
        .map(|(y, x)| y * 100 + x)
        .sum()
}

harness!(part_1: 1383666, part_2: 1412866);
