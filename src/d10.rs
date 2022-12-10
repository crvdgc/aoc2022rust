fn run_line_part0(line: &str, x: &mut i32, sigs: &mut Vec<i32>) {
    if let Some(value) = line.strip_prefix("addx ") {
        let v: i32 = value.parse().unwrap();
        sigs.push(*x);
        sigs.push(*x);
        *x += v;
    } else {
        // noop
        sigs.push(*x);
    }
}
pub fn part0(input: String) {
    let mut x = 1;
    let mut sigs = Vec::new();
    for line in input.lines() {
        run_line_part0(line, &mut x, &mut sigs);
    }
    let is: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
    let ans: i32 = is.iter().map(|i| sigs[*i - 1] * (*i as i32)).sum();
    println!("{}", ans);
}

fn draw_sigs(sigs: &Vec<i32>) {
    for (i, x) in sigs.iter().enumerate() {
        let cycle = (i + 1) as i32;
        let col = cycle % 40 - 1;
        if (col - x).abs() <= 1 {
            print!("#");
        } else {
            print!(".");
        }
        if cycle % 40 == 0 {
            print!("\n");
        }
    }
}
pub fn part1(input: String) {
    let mut x = 1;
    let mut sigs = Vec::new();
    for line in input.lines() {
        run_line_part0(line, &mut x, &mut sigs);
    }
    draw_sigs(&sigs);
}

pub fn short_example_input() -> String {
    r#"noop
addx 3
addx -5"#
        .to_string()
}
pub fn example_input() -> String {
    r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#
        .to_string()
}
