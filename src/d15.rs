type Pos = (i64, i64);

fn parse_pos(s: &str) -> Pos {
    let mut iter = s.split(", ");
    let x = iter
        .next()
        .unwrap()
        .strip_prefix("x=")
        .unwrap()
        .parse()
        .unwrap();
    let y = iter
        .next()
        .unwrap()
        .strip_prefix("y=")
        .unwrap()
        .parse()
        .unwrap();
    (x, y)
}

fn parse_line(line: &str) -> (Pos, Pos) {
    let mut iter = line.split(": ");

    let sensor = iter.next().unwrap().strip_prefix("Sensor at ").unwrap();
    let sensor = parse_pos(sensor);

    let beacon = iter
        .next()
        .unwrap()
        .strip_prefix("closest beacon is at ")
        .unwrap();
    let beacon = parse_pos(beacon);

    (sensor, beacon)
}

type Seg = (i64, i64);

fn manhattan_dist((x1, y1): Pos, (x2, y2): Pos) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn solve_y((sx, sy): Pos, (bx, by): Pos, y: i64) -> Option<Seg> {
    let dy = (sy - y).abs();
    let dsb = manhattan_dist((sx, sy), (bx, by));
    if dsb < dy {
        None
    } else {
        let dx = dsb - dy;
        Some((sx - dx, sx + dx))
    }
}

fn len_union_segs(mut segs: Vec<Seg>) -> i64 {
    let mut l: i64 = 0;
    segs.sort_unstable();
    let mut right_most = segs[0].0 - 1;
    for (x, y) in segs.iter() {
        if *x >= right_most {
            l += *y - *x;
            right_most = *y;
        } else if *y > right_most {
            l += *y - right_most;
            right_most = *y;
        }
    }
    l
}

pub fn part0(input: &str, y: i64) {
    let segs: Vec<Pos> = input
        .lines()
        .map(parse_line)
        .filter_map(|(p1, p2)| solve_y(p1, p2, y))
        .collect();
    let ans = len_union_segs(segs);
    println!("{}", ans);
    // dbg!(&pairs);
}

fn find_first_available(mut segs: Vec<Seg>) -> i64 {
    segs.sort_unstable();
    let mut right_most = segs[0].0 - 1;
    for (x, y) in segs.iter() {
        if *x > right_most + 1 {
            if right_most + 1 >= 0 {
                return right_most + 1;
            } else {
                right_most = *y;
            }
        } else if *y > right_most {
            right_most = *y;
        }
    }
    right_most + 1
}

pub fn part1(input: &str, x_limit: i64, y_limit: i64) {
    let pairs: Vec<(Pos, Pos)> = input.lines().map(parse_line).collect();
    for y in 0..y_limit {
        let segs: Vec<Seg> = pairs
            .iter()
            .filter_map(|(p1, p2)| solve_y(*p1, *p2, y))
            .collect();
        let x = find_first_available(segs);
        if x >= 0 && x < x_limit {
            // println!("{}, {}", x, y);
            let freq = x * 4000000 + y;
            println!("{}", freq);
            break;
        }
    }
}

pub fn example_input() -> &'static str {
    r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#
}
