pub fn part0(input: &str) {
    let v = parse(input);
    // dbg!(v);
    let ans = snafu_of_v(v);
    println!("{}", ans);
}

pub fn part1(input: &str) {}

fn v_of_snafu_digit(c: char) -> i64 {
    match c {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => panic!("Unknown char: {}", c),
    }
}

fn snafu_of_v(n: i64) -> String {
    let mut chars: Vec<char> = Vec::new();
    let mut n = n;
    while n != 0 {
        let mut div = n.div_euclid(5);
        let rem = n.rem_euclid(5);
        match rem {
            0 => {
                chars.push('0');
            }
            1 => {
                chars.push('1');
            }
            2 => {
                chars.push('2');
            }
            3 => {
                chars.push('=');
                div += 1;
            }
            4 => {
                chars.push('-');
                div += 1;
            }
            _ => unreachable!(),
        }
        n = div;
    }
    chars.into_iter().rev().collect()
}

fn parse(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            line.chars()
                .rev()
                .enumerate()
                .map(|(n, c)| 5_i64.pow(n as u32) * v_of_snafu_digit(c))
                .sum::<i64>()
        })
        .sum()
}

pub fn example_input() -> &'static str {
    r#"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"#
}
