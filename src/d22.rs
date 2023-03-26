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
        todo!()
    }
}

pub fn part0(input: &str, width: usize) {
    let (map, insts) = parse_input(input, width);
    pp_map(&map);
    pp_inst(&insts);
}

pub fn part1(input: &str, width: usize, block_n: usize) {}

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
        line.replace('L', " L ")
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
