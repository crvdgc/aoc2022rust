type Point = (usize, usize);

fn parse_rock(line: &str) -> Vec<Point> {
    fn parse_point(p: &str) -> Point {
        let mut iter = p.split(',');
        let x = iter.next().unwrap().parse().unwrap();
        let y = iter.next().unwrap().parse().unwrap();
        (x, y)
    }
    line.split(" -> ").map(parse_point).collect()
}

fn min_max<T>(x1: T, x2: T) -> (T, T)
where
    T: Ord + Copy,
{
    (Ord::min(x1, x2), Ord::max(x1, x2))
}

fn mark_points(grid: &mut [Vec<bool>], (x1, y1): Point, (x2, y2): Point) {
    if x1 == x2 {
        let (min_y, max_y) = min_max(y1, y2);
        for y in min_y..=max_y {
            grid[x1][y] = true;
        }
    } else if y1 == y2 {
        let (min_x, max_x) = min_max(x1, x2);
        (min_x..=max_x).for_each(|x| {
            grid[x][y1] = true;
        });
    } else {
        panic!("not aligned")
    }
}

fn mark_rock(grid: &mut [Vec<bool>], rock: &Vec<Point>) {
    for i in 0..(rock.len() - 1) {
        mark_points(grid, rock[i], rock[i + 1]);
    }
}

fn fst<A, B>((a, _b): (A, B)) -> A {
    a
}

fn snd<A, B>((_a, b): (A, B)) -> B {
    b
}

fn print_grid(grid: &[Vec<bool>]) {
    for row in grid.iter() {
        for p in row.iter() {
            if *p {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()
    }
}

enum Fall {
    Rest(Point),
    Void,
    Change(Point),
}

fn fall(grid: &[Vec<bool>], limit: usize, (x, y): Point) -> Fall {
    if y + 1 >= limit {
        Fall::Void
    } else {
        let ny = y + 1;
        if grid[x][ny] {
            if grid[x - 1][ny] {
                if grid[x + 1][ny] {
                    Fall::Rest((x, y))
                } else {
                    Fall::Change((x + 1, ny))
                }
            } else {
                Fall::Change((x - 1, ny))
            }
        } else {
            Fall::Change((x, ny))
        }
    }
}

fn run_sand(grid: &mut [Vec<bool>], limit: usize) -> u32 {
    let mut p: Point = (500, 0);
    let mut counter: u32 = 0;
    loop {
        match fall(grid, limit, p) {
            Fall::Rest((x, y)) => {
                counter += 1;
                grid[x][y] = true;
                p = (500, 0);
            }
            Fall::Void => {
                return counter;
            }
            Fall::Change(np) => p = np,
        }
    }
}

pub fn part0(input: &str) {
    let rocks: Vec<Vec<Point>> = input.lines().map(parse_rock).collect();
    let max_x = rocks
        .iter()
        .map(|rock| rock.iter().map(|&x| fst(x)).max().unwrap())
        .max()
        .unwrap();
    let max_y = rocks
        .iter()
        .map(|rock| rock.iter().map(|&x| snd(x)).max().unwrap())
        .max()
        .unwrap();
    // dbg!(max_x);
    // dbg!(max_y);
    let mut grid = vec![vec![false; max_y + 2]; max_x + 2];
    for rock in rocks {
        mark_rock(&mut grid, &rock);
    }
    // print_grid(&grid)
    let ans = run_sand(&mut grid, max_y + 1);
    // print_grid(&grid);
    println!("{}", ans);
}

fn run_sand_part1(grid: &mut [Vec<bool>], limit: usize) -> u32 {
    let mut p: Point = (500, 0);
    let mut counter: u32 = 0;
    loop {
        match fall(grid, limit, p) {
            Fall::Rest((x, y)) => {
                if (x, y) == (500, 0) {
                    return counter;
                } else {
                    counter += 1;
                    grid[x][y] = true;
                    p = (500, 0);
                }
            }
            Fall::Void => {
                panic!("Should not return void")
            }
            Fall::Change(np) => p = np,
        }
    }
}

pub fn part1(input: &str) {
    let rocks: Vec<Vec<Point>> = input.lines().map(parse_rock).collect();
    let max_x = rocks
        .iter()
        .map(|rock| rock.iter().map(|&x| fst(x)).max().unwrap())
        .max()
        .unwrap();
    let max_y = rocks
        .iter()
        .map(|rock| rock.iter().map(|&x| snd(x)).max().unwrap())
        .max()
        .unwrap();
    let mut grid = vec![vec![false; max_y + 4]; max_x + 1000];
    for rock in rocks {
        mark_rock(&mut grid, &rock);
    }
    // mark ground
    (0..max_x + 1000).for_each(|x| {
        grid[x][max_y + 2] = true;
    });
    // print_grid(&grid)
    let ans = run_sand_part1(
        &mut grid,
        max_y + 100, // no limit
    );
    // print_grid(&grid);
    println!("{}", ans + 1);
}

pub fn example_input() -> &'static str {
    r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#
}
