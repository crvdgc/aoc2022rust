pub fn part0(input: &str, width: usize) {
    let (map, insts) = parse_input(input, width);
    // pp_map(&map);
    // pp_inst(&insts);
    // let map_t = transpose(&map, width);
    let point_movements = calc_movement(&map, width);
    let mut p = (0, start_c(&map[0]));
    let mut dir = Dir::Right;
    for inst in insts {
        match inst {
            Inst::Move(n) => {
                for _ in 0..n {
                    let movement = &point_movements[p.0][p.1].expect("invalid start");
                    match movement.step(&dir) {
                        None => {
                            break;
                        }
                        Some(next) => {
                            p = next;
                        }
                    }
                }
            }
            Inst::Rotate(turn) => {
                dir = dir.turn(turn);
            }
        }
    }
    let password = (p.0 + 1) * 1000 + (p.1 + 1) * 4 + dir.value();
    println!("{}", password);
}

#[derive(Debug)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
}

enum Turn {
    L,
    R,
}

impl Turn {
    fn to_char(&self) -> char {
        match &self {
            Self::L => 'L',
            Self::R => 'R',
        }
    }

    fn of_char(c: char) -> Self {
        match c {
            'L' => Self::L,
            'R' => Self::R,
            c => panic!("Unknown turn {}", c),
        }
    }
}

impl Dir {
    fn turn(&self, t: Turn) -> Dir {
        match (t, self) {
            (Turn::L, Dir::Up) => Dir::Left,
            (Turn::L, Dir::Left) => Dir::Down,
            (Turn::L, Dir::Down) => Dir::Right,
            (Turn::L, Dir::Right) => Dir::Up,
            (Turn::R, Dir::Up) => Dir::Right,
            (Turn::R, Dir::Left) => Dir::Up,
            (Turn::R, Dir::Down) => Dir::Left,
            (Turn::R, Dir::Right) => Dir::Down,
        }
    }

    fn value(&self) -> usize {
        match self {
            Dir::Up => 3,
            Dir::Left => 2,
            Dir::Down => 1,
            Dir::Right => 0,
        }
    }
}

enum Inst {
    Move(usize),
    Rotate(Turn),
}

fn parse_input(input: &str, width: usize) -> (Vec<Vec<Option<bool>>>, Vec<Inst>) {
    fn parse_line(line: &str, width: usize) -> Vec<Option<bool>> {
        let mut v: Vec<Option<bool>> = line
            .chars()
            .map(|c| match c {
                ' ' => None,
                '.' => Some(false),
                '#' => Some(true),
                _ => panic!("Unknown char: {}", c),
            })
            .collect();
        v.append(&mut vec![None; width - v.len()]);
        v
    }
    let mut iter = input.split("\n\n");
    let map = iter
        .next()
        .unwrap()
        .lines()
        .map(|line| parse_line(line, width))
        .collect();
    fn parse_path(line: &str) -> Vec<Inst> {
        line.trim()
            .replace('L', " L ")
            .replace('R', " R ")
            .split(' ')
            .map(|s| {
                if let Ok(n) = s.parse::<usize>() {
                    Inst::Move(n)
                } else {
                    Inst::Rotate(Turn::of_char(s.chars().next().unwrap()))
                }
            })
            .collect()
    }
    let path = parse_path(iter.next().unwrap());
    (map, path)
}

pub fn pp_map(map: &[Vec<Option<bool>>]) {
    map.iter().for_each(|line| {
        line.iter()
            .map(|v| match v {
                Some(false) => '.',
                Some(true) => '#',
                None => ' ',
            })
            .for_each(|c| print!("{}", c));
        println!();
    })
}

fn pp_inst(insts: &[Inst]) {
    insts.iter().for_each(|inst| match inst {
        Inst::Move(n) => print!("{} ", n),
        Inst::Rotate(turn) => print!("{} ", turn.to_char()),
    })
}

fn col(map: &[Vec<Option<bool>>], c: usize) -> Vec<Option<bool>> {
    map.iter().map(|row| row[c]).collect()
}

fn transpose(map: &[Vec<Option<bool>>], width: usize) -> Vec<Vec<Option<bool>>> {
    (0..width).map(|c| col(map, c)).collect()
}

fn next_up(map_t: &[Vec<Option<bool>>], p: (usize, usize)) -> Option<(usize, usize)> {
    let col = &map_t[p.1];
    // dbg!(col);
    let l = col.len();
    for dr in 1..l {
        let r = (p.0 + l - dr) % l;
        match col[r] {
            Some(true) => {
                return None;
            }
            Some(false) => {
                return Some((r, p.1));
            }
            None => {}
        }
    }
    panic!("All none")
}

fn next_down(map_t: &[Vec<Option<bool>>], p: (usize, usize)) -> Option<(usize, usize)> {
    let col = &map_t[p.1];
    // dbg!(col);
    let l = col.len();
    for dr in 1..l {
        let r = (p.0 + dr) % l;
        match col[r] {
            Some(true) => {
                return None;
            }
            Some(false) => {
                return Some((r, p.1));
            }
            None => {}
        }
    }
    panic!("All none")
}
fn next_left(map: &[Vec<Option<bool>>], p: (usize, usize)) -> Option<(usize, usize)> {
    let row = &map[p.0];
    let l = row.len();
    for dc in 1..l {
        let c = (p.1 + l - dc) % l;
        match row[c] {
            Some(true) => {
                return None;
            }
            Some(false) => {
                return Some((p.0, c));
            }
            None => {}
        }
    }
    panic!("All none")
}

fn next_right(map: &[Vec<Option<bool>>], p: (usize, usize)) -> Option<(usize, usize)> {
    let row = &map[p.0];
    let l = row.len();
    for dc in 1..l {
        let c = (p.1 + dc) % l;
        match row[c] {
            Some(true) => {
                return None;
            }
            Some(false) => {
                return Some((p.0, c));
            }
            None => {}
        }
    }
    panic!("All none")
}

#[derive(Clone, Copy)]
struct PointMovement {
    up: Option<(usize, usize)>,
    left: Option<(usize, usize)>,
    down: Option<(usize, usize)>,
    right: Option<(usize, usize)>,
}

impl PointMovement {
    fn step(&self, dir: &Dir) -> Option<(usize, usize)> {
        match dir {
            Dir::Up => self.up,
            Dir::Left => self.left,
            Dir::Down => self.down,
            Dir::Right => self.right,
        }
    }
}

fn calc_movement(map: &[Vec<Option<bool>>], width: usize) -> Vec<Vec<Option<PointMovement>>> {
    let map_t = transpose(map, width);
    (0..map.len())
        .map(|r| {
            (0..width)
                .map(|c| {
                    map[r][c].and_then(|filled| {
                        if filled {
                            None
                        } else {
                            Some(PointMovement {
                                up: next_up(&map_t, (r, c)),
                                left: next_left(map, (r, c)),
                                down: next_down(&map_t, (r, c)),
                                right: next_right(map, (r, c)),
                            })
                        }
                    })
                })
                .collect()
        })
        .collect()
}

fn start_c(first_row: &[Option<bool>]) -> usize {
    first_row
        .iter()
        .position(|p| matches!(p, Some(false)))
        .expect("No open")
}

// ┌───┐  ┌─┐
// │   1122 │
// │┌──1122┐│
// ││  33┘ ││
// ││ ┌33  ││
// │└5544──┘│
// │ 5544   │
// └─66┘    │
//   66     │
//    └─────┘
pub fn part1(input: &str, width: usize, block_n: usize) {}

pub fn example_input() -> &'static str {
    r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"#
}
