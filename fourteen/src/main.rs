use advent::prelude::*;

#[derive(HasParser, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[parse(sep_by = ",")]
struct Point {
    x: i64,
    y: i64,
}

#[derive(HasParser)]
struct Robot {
    #[parse(before = "p=")]
    p: Point,
    #[parse(before = "v=")]
    v: Point,
}

fn modulo(v: i64, m: i64) -> i64 {
    if v < 0 {
        let v = v % m;
        if v < 0 {
            v + m
        } else {
            v
        }
    } else {
        v % m
    }
}

#[test]
fn modulo_test() {
    assert_eq!(modulo(14, 10), 4);
    assert_eq!(modulo(-3, 10), 7);
    assert_eq!(modulo(-13, 10), 7);
    assert_eq!(modulo(-7, 7), 0);
}

fn simulate_second(
    robots: &BTreeMap<Point, Vec<Point>>,
    width: i64,
    height: i64,
) -> BTreeMap<Point, Vec<Point>> {
    let mut out = BTreeMap::<Point, Vec<Point>>::new();
    for (p, robots) in robots {
        for r in robots {
            let mut np = *p;
            np.x = modulo(np.x + r.x, width);
            np.y = modulo(np.y + r.y, height);
            assert!(np.x >= 0 && np.x < width);
            assert!(
                np.y >= 0 && np.y < height,
                "{}, {} = {}",
                p.y + r.y,
                height,
                np.y
            );
            out.entry(np).or_default().push(*r);
        }
    }
    out
}

fn count_section(robots: &BTreeMap<Point, Vec<Point>>, s: Point, e: Point) -> usize {
    let mut total = 0;
    for (k, r) in robots {
        if k.x >= s.x && k.y >= s.y && k.x < e.x && k.y < e.y {
            total += r.len();
        }
    }
    total
}

#[part_one]
fn part_one(input: List<Robot, TermWith<NewLine>>) -> usize {
    let mut robots = BTreeMap::<Point, Vec<Point>>::new();
    for r in input {
        robots.entry(r.p).or_default().push(r.v);
    }

    let (width, height) = (101, 103);

    for _ in 0..100 {
        let new_robots = simulate_second(&robots, width, height);
        robots = new_robots;
    }

    let (m_x, m_y) = (width / 2, height / 2);
    let mut total = 1;
    total *= count_section(&robots, Point { x: 0, y: 0 }, Point { x: m_x, y: m_y });
    total *= count_section(
        &robots,
        Point { x: m_x + 1, y: 0 },
        Point { x: width, y: m_y },
    );
    total *= count_section(
        &robots,
        Point { x: 0, y: m_y + 1 },
        Point { x: m_x, y: height },
    );
    total *= count_section(
        &robots,
        Point {
            x: m_x + 1,
            y: m_y + 1,
        },
        Point {
            x: width,
            y: height,
        },
    );
    total
}

#[expect(dead_code)]
fn print_robots(robots: &BTreeMap<Point, Vec<Point>>, width: i64, height: i64) {
    for y in 0..height {
        for x in 0..width {
            if let Some(r) = robots.get(&Point { x, y }) {
                print!("{}", r.len());
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn diag_factor(robots: &BTreeMap<Point, Vec<Point>>) -> usize {
    let mut total = 0;
    for p1 in robots.keys() {
        for p2 in robots.keys() {
            if (p1.x + 1 == p2.x || p1.x - 1 == p2.x) && (p1.y + 1 == p2.y || p1.y - 1 == p2.y) {
                total += 1;
            }
        }
    }
    total
}

#[part_two]
fn part_two(input: List<Robot, TermWith<NewLine>>) -> usize {
    let mut robots = BTreeMap::<Point, Vec<Point>>::new();
    for r in input {
        robots.entry(r.p).or_default().push(r.v);
    }

    let (width, height) = (101, 103);

    for t in 0.. {
        let new_robots = simulate_second(&robots, width, height);
        robots = new_robots;

        if diag_factor(&robots) > 250 {
            return t + 1;
        }
    }
    unreachable!()
}

harness!(part_1: 217328832, part_2: 7412);
