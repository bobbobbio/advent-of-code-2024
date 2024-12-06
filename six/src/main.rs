use advent::prelude::*;
use std::fmt;

#[derive(HasParser, PartialEq, Eq, Copy, Clone)]
enum MapCell {
    #[parse(string = ".")]
    Empty,
    #[parse(string = "#")]
    Obstruction,
    #[parse(string = "^")]
    Guard,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum PathCell {
    Unvisited,
    Obstruction,
    Visited,
}

impl fmt::Display for PathCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unvisited => write!(f, "."),
            Self::Obstruction => write!(f, "#"),
            Self::Visited => write!(f, "X"),
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
enum Direction {
    North,
    South,
    East,
    West
}

fn path_from_grid(input: Grid<MapCell>) -> Grid<PathCell> {
    Grid::new(input.rows().map(|r| r.iter().map(|c| match c {
        MapCell::Empty => PathCell::Unvisited,
        MapCell::Obstruction => PathCell::Obstruction,
        MapCell::Guard => PathCell::Visited,
    }).collect()).collect()).unwrap()
}

fn walk(path: &mut Grid<PathCell>, direction: &mut Direction, y: &mut usize, x: &mut usize) -> bool {
    path[*y][*x] = PathCell::Visited;

    match *direction {
        Direction::North => {
            if *y == 0 {
                return true;
            }
            if path[*y - 1][*x] == PathCell::Obstruction {
                *direction = Direction::East;
                return false;
            }
            *y -= 1;
        },
        Direction::South => {
            if *y == path.height() - 1 {
                return true;
            }
            if path[*y + 1][*x] == PathCell::Obstruction {
                *direction = Direction::West;
                return false;
            }
                *y += 1;
        },
        Direction::East => {
            if *x == path.width() - 1 {
                return true;
            }

            if path[*y][*x + 1] == PathCell::Obstruction {
                *direction = Direction::South;
                return false;
            }
            *x += 1;
        },
        Direction::West => {
            if *x == 0 {
                return true;
            }
            if path[*y][*x - 1] == PathCell::Obstruction {
                *direction = Direction::North;
                return false;
            }
            *x -= 1;
        },
    }
    false
}

fn num_visited(path: Grid<PathCell>) -> usize {
    path.cells().filter(|&&c| c == PathCell::Visited).count()
}

fn part_one_inner(input: Grid<MapCell>) -> Grid<PathCell> {
    let (mut y, mut x) = input.position(|&c| c == MapCell::Guard).unwrap();

    let mut path = path_from_grid(input);
    let mut direction = Direction::North;
    loop {
        if walk(&mut path, &mut direction, &mut y, &mut x) {
            break;
        }
    }
    path
}


#[part_one]
fn part_one(input: Grid<MapCell>) -> usize {
    let path = part_one_inner(input);
    num_visited(path)
}

fn stuck_in_loop(input: Grid<MapCell>) -> bool {
    let (mut y, mut x) = input.position(|&c| c == MapCell::Guard).unwrap();

    let mut path = path_from_grid(input);
    let mut direction = Direction::North;
    let mut visited = HashSet::new();
    loop {
        visited.insert((y, x, direction));
        if walk(&mut path, &mut direction, &mut y, &mut x) {
            return false;
        }
        if visited.contains(&(y, x, direction)) {
            return true;
        }
    }
}

#[part_two]
fn part_two(input: Grid<MapCell>) -> usize {
    let path = part_one_inner(input.clone());
    let mut total = 0;
    for y in 0..path.height() {
        for x in 0..path.width() {
            if path[y][x] == PathCell::Visited && input[y][x] == MapCell::Empty {
                let mut test_input = input.clone();
                test_input[y][x] = MapCell::Obstruction;
                if stuck_in_loop(test_input) {
                    total += 1;
                }
            }
        }
    }
    total
}

harness!(part_1: 5331, part_2: 1812);
