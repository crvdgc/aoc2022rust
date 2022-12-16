type Pos = (usize, usize);
type Grid<T> = Vec<Vec<T>>;

fn height(c: char) -> u32 {
    c as u32 - 'a' as u32
}

fn parse_map(input: &str) -> (Grid<u32>, Pos, Pos) {
    let mut start: Option<Pos> = None;
    let mut end: Option<Pos> = None;
    let mut map: Vec<Vec<u32>> = Vec::new();
    for (r, line) in input.lines().enumerate() {
        let mut row: Vec<u32> = Vec::new();
        for (c, ch) in line.chars().enumerate() {
            match ch {
                'S' => {
                    row.push(height('a'));
                    start = Some((r, c));
                }
                'E' => {
                    row.push(height('z'));
                    end = Some((r, c));
                }
                _ => row.push(height(ch)),
            }
        }
        map.push(row)
    }
    (map, start.unwrap(), end.unwrap())
}

fn neighbors((x, y): Pos, w: usize, h: usize) -> Vec<Pos> {
    let mut v = Vec::with_capacity(4);
    if x > 0 {
        v.push((x - 1, y));
    }
    if y > 0 {
        v.push((x, y - 1));
    }
    if x + 1 < w {
        v.push((x + 1, y));
    }
    if y + 1 < h {
        v.push((x, y + 1));
    }
    v
}

fn solve_one(map: &Grid<u32>, steps: &mut Grid<Option<u32>>, w: usize, h: usize) {
    for r in 0..w {
        for c in 0..h {
            if steps[r][c].is_none() {
                let cur_height = map[r][c];
                let best_neighbor: Option<u32> = neighbors((r, c), w, h)
                    .iter()
                    .filter_map(|&(nr, nc)| {
                        steps[nr][nc].and_then(|s| {
                            if map[nr][nc] <= cur_height + 1 {
                                Some(s)
                            } else {
                                None
                            }
                        })
                    })
                    .min();
                if let Some(s) = best_neighbor {
                    steps[r][c] = Some(s + 1);
                }
            }
        }
    }
}

pub fn part0(input: &str) {
    let (map, (start_r, start_c), (end_r, end_c)) = parse_map(input);
    // dbg!(&map);
    let w = map.len();
    let h = map[0].len();
    let mut steps = vec![vec![None; h]; w];
    steps[end_r][end_c] = Some(0);
    for _ in 0..(w * h) {
        solve_one(&map, &mut steps, w, h);
    }
    // dbg![&steps];
    println!("{}", steps[start_r][start_c].unwrap());
}
pub fn part1(input: &str) {
    let (map, (_start_r, _start_c), (end_r, end_c)) = parse_map(input);
    // dbg!(&map);
    let w = map.len();
    let h = map[0].len();
    let mut steps = vec![vec![None; h]; w];
    steps[end_r][end_c] = Some(0);
    for _ in 0..(w * h) {
        solve_one(&map, &mut steps, w, h);
    }
    // dbg![&steps];
    let candidates: Vec<Pos> = map
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(c, h)| {
                    if *h == height('a') {
                        Some((r, c))
                    } else {
                        None
                    }
                })
                .collect::<Vec<Pos>>()
        })
        .collect();
    let best = candidates
        .iter()
        .filter_map(|(cr, cc)| steps[*cr][*cc])
        .min()
        .unwrap();
    println!("{}", best);
}

pub fn example_input() -> &'static str {
    r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#
}
