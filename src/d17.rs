type P = (usize, usize);
const WIDTH: usize = 7;
const MAX_BLOCK_HEIGHT: usize = 4;

enum Dir {
    Left,
    Right,
}

fn pp((x, y): P) {
    println!("({}, {})", x, y)
}

/// Origin (x, y) = (0, 0) is left bottom
/// x axis goes up, y axis goes right
struct Block {
    left: Vec<P>,   // left most points to check for collision to the left
    right: Vec<P>,  // same, but for right
    bottom: Vec<P>, // same, but for bottom
    points: Vec<P>, // all points to fill the tunnel
    width: usize,
    height: usize,
}

fn blocks() -> Vec<Block> {
    vec![
        // ####
        Block {
            left: vec![(0, 0)],
            right: vec![(0, 3)],
            bottom: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            points: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            width: 4,
            height: 1,
        },
        // .#.
        // ###
        // .#.
        Block {
            left: vec![(0, 1), (1, 0), (2, 1)],
            right: vec![(0, 1), (1, 2), (2, 1)],
            bottom: vec![(1, 0), (0, 1), (1, 2)],
            points: vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
            width: 3,
            height: 3,
        },
        // ..#
        // ..#
        // ###
        Block {
            left: vec![(0, 0), (1, 2), (2, 2)],
            right: vec![(0, 2), (1, 2), (2, 2)],
            bottom: vec![(0, 0), (0, 1), (0, 2)],
            points: vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
            width: 3,
            height: 3,
        },
        // #
        // #
        // #
        // #
        Block {
            left: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            right: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            bottom: vec![(0, 0)],
            points: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            width: 1,
            height: 4,
        },
        // ##
        // ##
        Block {
            left: vec![(0, 0), (1, 0)],
            right: vec![(0, 1), (1, 1)],
            bottom: vec![(0, 0), (0, 1)],
            points: vec![(0, 0), (0, 1), (1, 0), (1, 1)],
            width: 2,
            height: 2,
        },
    ]
}

fn collision_left(tunnel: &[[bool; WIDTH]], block: &Block, (x, y): P) -> bool {
    y == 0 || block.left.iter().any(|(dx, dy)| tunnel[x + dx][y + dy - 1])
}

fn collision_right(tunnel: &[[bool; WIDTH]], block: &Block, (x, y): P) -> bool {
    y + block.width >= WIDTH
        || block
            .right
            .iter()
            .any(|(dx, dy)| tunnel[x + dx][y + dy + 1])
}

fn collision_bottom(tunnel: &[[bool; WIDTH]], block: &Block, (x, y): P) -> bool {
    x == 0
        || block
            .bottom
            .iter()
            .any(|(dx, dy)| tunnel[x + dx - 1][y + dy])
}

fn fill(tunnel: &mut [[bool; WIDTH]], block: &Block, (x, y): P) {
    block
        .points
        .iter()
        .for_each(|(dx, dy)| tunnel[x + dx][y + dy] = true)
}

fn ensure_height(tunnel: &mut Vec<[bool; WIDTH]>, height: usize) {
    let target_height = height + 3 + MAX_BLOCK_HEIGHT;
    for _ in tunnel.len()..=target_height {
        tunnel.push([false; WIDTH]);
    }
}

fn start_pos(height: usize) -> P {
    (height + 3, 2)
}

fn parse_jet_pattern(s: &str) -> Vec<Dir> {
    s.chars()
        .map(|c| match c {
            '<' => Dir::Left,
            '>' => Dir::Right,
            c => panic!("Unknown char in jet pattern: {:?}", c),
        })
        .collect()
}

fn print_tunnel(tunnel: &[[bool; WIDTH]], height: usize) {
    for line in tunnel[..height + 3].iter().rev() {
        for c in line {
            let c = if *c { '#' } else { '.' };
            print!("{}", c);
        }
        println!();
    }
    println!();
}

pub fn part0(input: &str) {
    let blocks = blocks();
    let mut blocks = blocks.iter().cycle();
    let patterns = parse_jet_pattern(input.trim());
    let mut patterns = patterns.iter().cycle();
    let mut tunnel: Vec<[bool; WIDTH]> = Vec::with_capacity(2023 * MAX_BLOCK_HEIGHT);
    let mut height: usize = 0;
    let mut count: usize = 0;
    while count < 2022 {
        ensure_height(&mut tunnel, height);
        let block = blocks.next().unwrap();
        let mut p: P = start_pos(height);
        // print!("new: ");
        // pp(p);
        loop {
            let pattern = patterns.next().unwrap();
            match pattern {
                Dir::Left => {
                    if !collision_left(&tunnel, block, p) {
                        p.1 -= 1
                    }
                }
                Dir::Right => {
                    if !collision_right(&tunnel, block, p) {
                        p.1 += 1
                    }
                }
            }
            if collision_bottom(&tunnel, block, p) {
                break;
            } else {
                p.0 -= 1;
            }
            // pp(p);
        }
        // print!("rest: ");
        // pp(p);
        fill(&mut tunnel, block, p);
        height = std::cmp::max(height, p.0 + block.height);
        // print_tunnel(&tunnel, height);
        count += 1;
    }
    println!("{}", height);
}

pub fn part1(input: &str) {}

pub fn example_input() -> &'static str {
    r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#
}
