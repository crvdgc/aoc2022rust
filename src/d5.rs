fn parse_crates(input: &str) -> Vec<Vec<char>> {
    let rows: Vec<&str> = input.lines().collect();
    let width: usize = rows.iter().map(|x| x.len()).max().unwrap();
    let mut cols: Vec<Vec<char>> = Vec::with_capacity(width);
    let mut i: usize = 1;
    while i < width {
        let col: Vec<char> = rows
            .iter()
            .map(|x| x.chars().skip(i).next().unwrap())
            .collect();
        cols.push(col);
        i += 4;
    }
    for col in cols.iter_mut() {
        let filtered: Vec<char> = col
            .iter()
            .take(col.len() - 1)
            .filter(|x| **x != ' ')
            .map(|x| x.clone())
            .rev()
            .collect();
        *col = filtered;
    }
    cols
}

struct CraneMove(usize, usize);

fn parse_move(line: &str) -> (usize, CraneMove) {
    let mut iter = line.split(" ").filter_map(|x| x.parse::<usize>().ok());
    let repeat: usize = iter.next().unwrap();
    let from: usize = iter.next().unwrap();
    let to: usize = iter.next().unwrap();
    (repeat, CraneMove(from - 1, to - 1))
}

fn parse(input: String) -> (Vec<Vec<char>>, Vec<(usize, CraneMove)>) {
    let mut iter = input.split("\n\n");
    let crates_input = iter.next().unwrap();
    let moves_input = iter.next().unwrap();
    let crates = parse_crates(crates_input);
    let moves = moves_input.lines().map(parse_move).collect();
    (crates, moves)
}

fn run_crane(crates: &mut Vec<Vec<char>>, moves: Vec<(usize, CraneMove)>) -> () {
    for (repeat, CraneMove(from, to)) in moves {
        for _ in 0..repeat {
            let x = crates[from].pop().expect("popping through bottom");
            crates[to].push(x);
        }
    }
}

pub fn part0(input: String) -> () {
    let (mut crates, moves) = parse(input);
    run_crane(&mut crates, moves);
    for c in crates {
        print!("{}", c.iter().last().unwrap());
    }
}

fn run_crane_same_order(crates: &mut Vec<Vec<char>>, moves: Vec<(usize, CraneMove)>) -> () {
    for (repeat, CraneMove(from, to)) in moves {
        let mut boxes = Vec::with_capacity(repeat);
        for _ in 0..repeat {
            boxes.push(crates[from].pop().unwrap());
        }
        boxes.reverse();
        crates[to].append(&mut boxes);
    }
}

pub fn part1(input: String) -> () {
    let (mut crates, moves) = parse(input);
    run_crane_same_order(&mut crates, moves);
    for c in crates {
        print!("{}", c.iter().last().unwrap());
    }
}
pub fn example_input() -> String {
    r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#
        .to_string()
}
