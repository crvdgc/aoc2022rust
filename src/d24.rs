use std::collections::{HashSet, VecDeque};

#[derive(Default, Clone, Copy)]
struct P {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

pub fn part0(input: &str) {
    let field = parse(input);
    let h = field.len();
    let w = field[0].len();
    let end = (h - 1, w - 1);
    let period = lcm(h, w);
    let mut ans = 0;
    let fields: Vec<Vec<Vec<P>>> = {
        let mut fields = Vec::with_capacity(period);
        fields.push(field);
        (1..period).for_each(|t| fields.push(step(h, w, &fields[t - 1])));
        fields
    };
    // (0..period).for_each(|t| {
    //     println!("{}", t);
    //     pp(&fields[t]);
    // });
    // panic!();
    let mut visited: HashSet<(usize, Option<(usize, usize)>)> =
        HashSet::with_capacity(period * w * h);
    let mut q: VecDeque<(usize, Option<(usize, usize)>)> = VecDeque::new();
    q.push_back((0, None));
    while let Some((time, pos)) = q.pop_front() {
        let time = time + 1;
        let st = time % period;
        let field = &fields[st];
        match pos {
            None => {
                // start
                let target = Some((0, 0));
                if field[0][0].is_empty() && !visited.contains(&(st, target)) {
                    q.push_back((time, target));
                }
                q.push_back((time, None));
            }
            Some((x, y)) => {
                if (x, y) == end {
                    ans = time;
                    break;
                } else {
                    if field[x][y].is_empty() {
                        q.push_back((time, Some((x, y))));
                    }
                    for (x, y) in neighbors(h, w, (x, y)) {
                        let target = Some((x, y));
                        if field[x][y].is_empty() && !visited.contains(&(st, target)) {
                            visited.insert((st, target));
                            q.push_back((time, target));
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans);
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Pos {
    Start,
    End,
    At(usize, usize),
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    pos: Pos,
    has_reached: bool,
    has_fetched: bool,
}

pub fn part1(input: &str) {
    let field = parse(input);
    let h = field.len();
    let w = field[0].len();
    let period = lcm(h, w);
    let mut ans = None;
    let fields: Vec<Vec<Vec<P>>> = {
        let mut fields = Vec::with_capacity(period);
        fields.push(field);
        (1..period).for_each(|t| fields.push(step(h, w, &fields[t - 1])));
        fields
    };
    let mut visited: HashSet<(usize, State)> = HashSet::with_capacity(period * w * h);
    let mut q: VecDeque<(usize, State)> = VecDeque::new();
    q.push_back((
        0,
        State {
            pos: Pos::Start,
            has_reached: false,
            has_fetched: false,
        },
    ));
    let try_push = |q: &mut VecDeque<(usize, State)>,
                    visited: &mut HashSet<(usize, State)>,
                    time,
                    target: State| {
        let st = time % period;
        let field: &Vec<Vec<P>> = &fields[st];
        let is_empty = match target.pos {
            Pos::At(x, y) => field[x][y].is_empty(),
            _ => true,
        };
        if is_empty && !visited.contains(&(st, target)) {
            visited.insert((st, target));
            q.push_back((time, target));
        }
    };
    while let Some((time, state)) = q.pop_front() {
        let time = time + 1;
        match state.pos {
            Pos::Start => {
                // start
                try_push(
                    &mut q,
                    &mut visited,
                    time,
                    State {
                        pos: Pos::At(0, 0),
                        ..state
                    },
                );
            }
            Pos::End => {
                assert!(state.has_reached);
                if state.has_fetched {
                    ans = Some(time - 1);
                    break;
                }
                try_push(
                    &mut q,
                    &mut visited,
                    time,
                    State {
                        pos: Pos::At(h - 1, w - 1),
                        ..state
                    },
                );
            }
            Pos::At(x, y) => {
                for (x, y) in neighbors(h, w, (x, y)) {
                    try_push(
                        &mut q,
                        &mut visited,
                        time,
                        State {
                            pos: Pos::At(x, y),
                            ..state
                        },
                    );
                }
                if (x, y) == (0, 0) {
                    try_push(
                        &mut q,
                        &mut visited,
                        time,
                        State {
                            pos: Pos::Start,
                            has_fetched: state.has_reached,
                            has_reached: state.has_reached,
                        },
                    );
                } else if (x, y) == (h - 1, w - 1) {
                    try_push(
                        &mut q,
                        &mut visited,
                        time,
                        State {
                            pos: Pos::End,
                            has_reached: true,
                            ..state
                        },
                    );
                }
            }
        };
        // not moving
        try_push(&mut q, &mut visited, time, state);
    }
    println!("{}", ans.unwrap());
}

fn step(h: usize, w: usize, field: &[Vec<P>]) -> Vec<Vec<P>> {
    let up = |(x, y): (usize, usize)| ((x + h - 1) % h, y);
    let down = |(x, y): (usize, usize)| ((x + 1) % h, y);
    let left = |(x, y): (usize, usize)| (x, (y + w - 1) % w);
    let right = |(x, y): (usize, usize)| (x, (y + 1) % w);
    let mut new_field: Vec<Vec<P>> = vec![vec![P::default(); w]; h];
    (0..h).for_each(|x| {
        (0..w).for_each(|y| {
            let c = (x, y);
            let p = field[x][y];
            if p.up {
                let (x, y) = up(c);
                new_field[x][y].up = true;
            }
            if p.down {
                let (x, y) = down(c);
                new_field[x][y].down = true;
            }
            if p.left {
                let (x, y) = left(c);
                new_field[x][y].left = true;
            }
            if p.right {
                let (x, y) = right(c);
                new_field[x][y].right = true;
            }
        })
    });
    // pp(&new_field);
    new_field
}

fn neighbors(h: usize, w: usize, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if x + 1 < h {
        neighbors.push((x + 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if y + 1 < w {
        neighbors.push((x, y + 1));
    }
    neighbors
}

impl P {
    fn is_empty(&self) -> bool {
        [self.up, self.down, self.left, self.right]
            .into_iter()
            .all(|is_occupied| !is_occupied)
    }
    fn from_char(c: char) -> Self {
        match c {
            '^' => Self {
                up: true,
                ..Self::default()
            },
            'v' => Self {
                down: true,
                ..Self::default()
            },
            '<' => Self {
                left: true,
                ..Self::default()
            },
            '>' => Self {
                right: true,
                ..Self::default()
            },
            '.' => Self::default(),
            c => panic!("Unknown mark {}", c),
        }
    }

    fn to_char(self) -> char {
        let n = [self.up, self.down, self.left, self.right]
            .into_iter()
            .filter(|x| *x)
            .count();
        if n == 0 {
            '.'
        } else if n == 1 {
            if self.up {
                '^'
            } else if self.down {
                'v'
            } else if self.left {
                '<'
            } else {
                '>'
            }
        } else {
            (n as u8 + b'0') as char
        }
    }
}

fn parse(input: &str) -> Vec<Vec<P>> {
    input
        .lines()
        .skip(1)
        .filter_map(|line| {
            if line.starts_with("##") {
                None
            } else {
                Some(
                    line.strip_prefix('#')
                        .unwrap()
                        .strip_suffix('#')
                        .unwrap()
                        .chars()
                        .map(P::from_char)
                        .collect(),
                )
            }
        })
        .collect()
}

fn pp(field: &Vec<Vec<P>>) {
    for row in field {
        for c in row {
            print!("{}", c.to_char());
        }
        println!();
    }
}

pub fn example_input() -> &'static str {
    r#"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"#
}

fn gcd(x: usize, y: usize) -> usize {
    use std::cmp::{max, min};
    let mut min = min(x, y);
    let mut max = max(x, y);
    loop {
        let rem = max % min;
        if rem == 0 {
            return min;
        }
        max = min;
        min = rem;
    }
}

fn lcm(x: usize, y: usize) -> usize {
    x * y / gcd(x, y)
}
