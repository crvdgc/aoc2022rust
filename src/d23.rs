use std::collections::HashMap;

type P = (i64, i64);

#[derive(Debug)]
struct BoundingBox {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

pub fn part0(input: &str) {
    let mut elves = parse(input);
    // pp(&elves);
    let proposals: [[(i64, i64); 3]; 4] = [
        [(-1, 0), (-1, -1), (-1, 1)],
        [(1, 0), (1, -1), (1, 1)],
        [(0, -1), (-1, -1), (1, -1)],
        [(0, 1), (-1, 1), (1, 1)],
    ];
    let mut proposal_i: usize = 0;
    for _ in 0..10 {
        let mut proposed_move: HashMap<P, Vec<usize>> = HashMap::with_capacity(elves.len());
        for i in 0..elves.len() {
            if let Some(p) = propose(&elves, i, proposals, proposal_i) {
                proposed_move
                    .entry(p)
                    .and_modify(|l| l.push(i))
                    .or_insert_with(|| vec![i]);
            }
        }
        for (p, l) in proposed_move {
            if l.len() == 1 {
                elves[l[0]] = p;
            }
        }
        proposal_i = (proposal_i + 1) % 4;
        // println!("{}, {}", elves[0].0, elves[0].1);
        // pp(&elves);
    }
    let bounding_box = bounding_box(&elves);
    // dbg!(&bounding_box);
    let empty_spaces = bounding_box.area() as usize - elves.len();
    println!("{}", empty_spaces);
}

pub fn part1(input: &str) {
    let mut elves = parse(input);
    // pp(&elves);
    let proposals: [[(i64, i64); 3]; 4] = [
        [(-1, 0), (-1, -1), (-1, 1)],
        [(1, 0), (1, -1), (1, 1)],
        [(0, -1), (-1, -1), (1, -1)],
        [(0, 1), (-1, 1), (1, 1)],
    ];
    let mut proposal_i: usize = 0;
    let mut round_n: usize = 0;
    loop {
        round_n += 1;
        let mut proposed_move: HashMap<P, Vec<usize>> = HashMap::with_capacity(elves.len());
        for i in 0..elves.len() {
            if let Some(p) = propose(&elves, i, proposals, proposal_i) {
                proposed_move
                    .entry(p)
                    .and_modify(|l| l.push(i))
                    .or_insert_with(|| vec![i]);
            }
        }
        let mut has_change = false;
        for (p, l) in proposed_move {
            if l.len() == 1 {
                elves[l[0]] = p;
                has_change = true;
            }
        }
        if !has_change {
            break;
        }
        proposal_i = (proposal_i + 1) % 4;
        // println!("{}, {}", elves[0].0, elves[0].1);
        // pp(&elves);
    }
    // let bounding_box = bounding_box(&elves);
    // dbg!(&bounding_box);
    // let empty_spaces = bounding_box.area() as usize - elves.len();
    println!("{}", round_n);
}

fn propose(elves: &[P], i: usize, proposals: [[(i64, i64); 3]; 4], proposal_i: usize) -> Option<P> {
    if !has_adjacent(elves, i) {
        return None;
    }
    let (x, y) = elves[i];
    for dpi in 0..4 {
        let proposal = proposals[(proposal_i + dpi) % 4];
        let is_ok = proposal
            .iter()
            .all(|(dx, dy)| !elves.contains(&(x + dx, y + dy)));
        if is_ok {
            return Some((x + proposal[0].0, y + proposal[0].1));
        }
    }
    None
}

fn has_adjacent(elves: &[P], i: usize) -> bool {
    let (x, y) = elves[i];
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            if elves.contains(&(x + dx, y + dy)) {
                return true;
            }
        }
    }
    false
}

fn pp(elves: &[P]) {
    let b = bounding_box(elves);
    for x in b.min_x..=b.max_x {
        for y in b.min_y..=b.max_y {
            if elves.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

impl BoundingBox {
    fn new() -> Self {
        BoundingBox {
            min_x: i64::MAX,
            max_x: i64::MIN,
            min_y: i64::MAX,
            max_y: i64::MIN,
        }
    }
    fn update(&mut self, (x, y): P) {
        use std::cmp::{max, min};
        self.min_x = min(self.min_x, x);
        self.max_x = max(self.max_x, x);
        self.min_y = min(self.min_y, y);
        self.max_y = max(self.max_y, y);
    }

    fn area(&self) -> i64 {
        (self.max_x - self.min_x + 1) * (self.max_y - self.min_y + 1)
    }
}

fn bounding_box(elves: &[P]) -> BoundingBox {
    let mut bounding_box = BoundingBox::new();
    for elf in elves {
        bounding_box.update(*elf);
    }
    bounding_box
}

fn parse(input: &str) -> Vec<P> {
    input
        .lines()
        .enumerate()
        .flat_map(|(r, row)| {
            row.chars()
                .enumerate()
                .filter_map(|(c, ch)| {
                    if ch == '#' {
                        Some((r as i64, c as i64))
                    } else {
                        None
                    }
                })
                .collect::<Vec<P>>()
        })
        .collect()
}

pub fn example_input() -> &'static str {
    r#"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."#
}
