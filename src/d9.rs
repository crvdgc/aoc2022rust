use std::collections::HashSet;

type Pos = (i32, i32);

fn next(dir: &str, (x, y): Pos) -> Pos {
    match dir {
        "U" => (x, y + 1),
        "D" => (x, y - 1),
        "L" => (x - 1, y),
        "R" => (x + 1, y),
        _ => panic!("Unknown direction"),
    }
}

fn neighbors((x, y): Pos) -> HashSet<Pos> {
    HashSet::from([
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x, y - 1),
        (x, y),
        (x, y + 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
    ])
}

fn dist_sq((x1, y1): Pos, (x2, y2): Pos) -> i32 {
    let dx = x1 - x2;
    let dy = y1 - y2;
    dx * dx + dy * dy
}

fn next_tail(head: Pos, tail: Pos) -> Pos {
    let head_neighbors = neighbors(head);
    if head_neighbors.contains(&tail) {
        tail
    } else {
        let tail_neighbors = neighbors(tail);
        let intersect = head_neighbors.intersection(&tail_neighbors);
        *intersect
            .into_iter()
            .min_by(|p1, p2| dist_sq(head, **p1).cmp(&dist_sq(head, **p2)))
            .unwrap()
    }
}

fn run_line(line: &str, head: &mut Pos, tail: &mut Pos) -> HashSet<Pos> {
    let mut iter = line.split(" ");
    let dir = iter.next().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();
    // dbg!(dir, n);
    // dbg!(&head, &tail);
    let mut trail: HashSet<Pos> = HashSet::new();
    trail.insert(*tail);
    for _ in 0..n {
        let new_head = next(dir, head.clone());
        *head = new_head;
        let new_tail = next_tail(head.clone(), tail.clone());
        trail.insert(new_tail);
        *tail = new_tail;
        // dbg!(&new_head, &new_tail);
    }
    trail
}

pub fn part0(input: String) {
    let mut trail: HashSet<Pos> = HashSet::new();
    let mut head: Pos = (0, 0);
    let mut tail: Pos = (0, 0);
    for line in input.lines() {
        let line_trail = run_line(line, &mut head, &mut tail);
        trail = trail.union(&line_trail).map(|x| *x).collect();
    }
    println!("{}", trail.len())
}

fn run_line_longer(line: &str, head: &mut Pos, tails: &mut Vec<Pos>) -> HashSet<Pos> {
    let mut iter = line.split(" ");
    let dir = iter.next().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut trail: HashSet<Pos> = HashSet::new();
    trail.insert(*tails.last().unwrap());
    for _ in 0..n {
        let new_head = next(dir, head.clone());
        *head = new_head;
        let mut cur_head = *head;
        for tail in tails.iter_mut() {
            let new_tail = next_tail(cur_head.clone(), tail.clone());
            *tail = new_tail;
            cur_head = new_tail;
        }
        trail.insert(*tails.last().unwrap());
        // dbg!(&new_head, &new_tail);
    }
    trail
}

pub fn part1(input: String) {
    let mut trail: HashSet<Pos> = HashSet::new();
    let mut head: Pos = (0, 0);
    let mut tails = vec![(0, 0); 9];
    for line in input.lines() {
        let line_trail = run_line_longer(line, &mut head, &mut tails);
        trail = trail.union(&line_trail).map(|x| *x).collect();
    }
    println!("{}", trail.len())
}
pub fn example_input() -> String {
    r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#
        .to_string()
}

pub fn example_input_larger() -> String {
    r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#
        .to_string()
}
