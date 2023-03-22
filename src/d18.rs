use std::collections::VecDeque;

type Coord = (usize, usize, usize);

pub fn part0(input: &str) {
    let (max, cubes, matrix) = parse_input(input);
    let n = cubes.len();
    let mut connected: usize = 0;
    for cube in cubes {
        for (x, y, z) in neighbors(max, cube) {
            if matrix[x][y][z] {
                connected += 1;
            }
        }
    }
    let surface_n = n * 6 - connected;
    println!("{}", surface_n);
}

pub fn part1(input: &str) {
    let (max, cubes, matrix) = parse_input(input);
    let mut queue: VecDeque<Coord> = VecDeque::new();
    let origin: Coord = (0, 0, 0);
    if cubes.contains(&origin) {
        panic!("origin is occupied");
    }
    queue.push_back(origin);
    let mut out_matrix = vec![vec![vec![false; max]; max]; max];
    let mut visited = vec![vec![vec![false; max]; max]; max];
    while let Some((x, y, z)) = queue.pop_front() {
        if matrix[x][y][z] {
            out_matrix[x][y][z] = true;
            visited[x][y][z] = true;
        } else {
            out_matrix[x][y][z] = false;
            visited[x][y][z] = true;
            for (x, y, z) in neighbors(max, (x, y, z)) {
                if !visited[x][y][z] && !queue.contains(&(x, y, z)) {
                    queue.push_back((x, y, z));
                }
            }
        }
    }
    let mut unreachable_n: usize = 0;
    let n = cubes.len();
    for (x, y, z) in cubes {
        for (x, y, z) in neighbors(max, (x, y, z)) {
            if !visited[x][y][z] || out_matrix[x][y][z] {
                unreachable_n += 1;
            }
        }
    }
    let out_surface_n = n * 6 - unreachable_n;
    println!("{}", out_surface_n);
}

fn neighbors(max: usize, (x, y, z): Coord) -> Vec<Coord> {
    let mut neighbors = Vec::with_capacity(6);
    if x + 1 < max {
        neighbors.push((x + 1, y, z));
    }
    if y + 1 < max {
        neighbors.push((x, y + 1, z));
    }
    if z + 1 < max {
        neighbors.push((x, y, z + 1));
    }
    if x > 0 {
        neighbors.push((x - 1, y, z));
    }
    if y > 0 {
        neighbors.push((x, y - 1, z));
    }
    if z > 0 {
        neighbors.push((x, y, z - 1));
    }
    neighbors
}

fn parse_input(input: &str) -> (usize, Vec<Coord>, Vec<Vec<Vec<bool>>>) {
    let mut cubes = Vec::with_capacity(3000);
    let mut max: usize = 0;
    for line in input.lines() {
        let mut c = line.split(',');
        let x: usize = c.next().unwrap().parse().unwrap();
        let y: usize = c.next().unwrap().parse().unwrap();
        let z: usize = c.next().unwrap().parse().unwrap();
        cubes.push((x, y, z));
        max = std::cmp::max(max, x);
        max = std::cmp::max(max, y);
        max = std::cmp::max(max, z);
    }
    max += 1;
    let mut matrix = vec![vec![vec![false; max]; max]; max];
    for &(x, y, z) in cubes.iter() {
        matrix[x][y][z] = true;
    }
    (max, cubes, matrix)
}

pub fn example_input() -> &'static str {
    r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"#
}
