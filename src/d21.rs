use std::collections::HashMap;

const N: usize = 26;

#[derive(Debug, Clone, Copy)]
enum Expr {
    Lit(i64),
    Add(usize, usize),
    Sub(usize, usize),
    Mul(usize, usize),
    Div(usize, usize),
}

fn dfs(root: usize, t: &HashMap<usize, Expr>) -> i64 {
    match t[&root] {
        Expr::Lit(n) => n,
        Expr::Add(e1, e2) => dfs(e1, t) + dfs(e2, t),
        Expr::Sub(e1, e2) => dfs(e1, t) - dfs(e2, t),
        Expr::Mul(e1, e2) => dfs(e1, t) * dfs(e2, t),
        Expr::Div(e1, e2) => dfs(e1, t) / dfs(e2, t),
    }
}

pub fn part0(input: &str) {
    let t = parse_input(input);
    // dbg!(&t);
    let res = dfs(name_to_index(&mut "root".chars()), &t);
    println!("{}", res);
}

fn dfs1(humn_n: usize, root: usize, t: &HashMap<usize, Expr>) -> Option<i64> {
    if root == humn_n {
        None
    } else {
        match t[&root] {
            Expr::Lit(n) => Some(n),
            Expr::Add(e1, e2) => match (dfs1(humn_n, e1, t), dfs1(humn_n, e2, t)) {
                (Some(v1), Some(v2)) => Some(v1 + v2),
                _ => None,
            },
            Expr::Sub(e1, e2) => match (dfs1(humn_n, e1, t), dfs1(humn_n, e2, t)) {
                (Some(v1), Some(v2)) => Some(v1 - v2),
                _ => None,
            },
            Expr::Mul(e1, e2) => match (dfs1(humn_n, e1, t), dfs1(humn_n, e2, t)) {
                (Some(v1), Some(v2)) => Some(v1 * v2),
                _ => None,
            },
            Expr::Div(e1, e2) => match (dfs1(humn_n, e1, t), dfs1(humn_n, e2, t)) {
                (Some(v1), Some(v2)) => Some(v1 / v2),
                _ => None,
            },
        }
    }
}

fn find(humn_n: usize, root: usize, t: &HashMap<usize, Expr>, target: i64) -> i64 {
    if root == humn_n {
        target
    } else {
        match t[&root] {
            Expr::Lit(_) => panic!("impossible"),
            Expr::Add(e1, e2) => match dfs1(humn_n, e1, t) {
                None => find(humn_n, e1, t, target - dfs(e2, t)),
                Some(v) => find(humn_n, e2, t, target - v),
            },
            Expr::Sub(e1, e2) => match dfs1(humn_n, e1, t) {
                None => find(humn_n, e1, t, target + dfs(e2, t)),
                Some(v) => find(humn_n, e2, t, v - target),
            },
            Expr::Mul(e1, e2) => match dfs1(humn_n, e1, t) {
                None => find(humn_n, e1, t, target / dfs(e2, t)),
                Some(v) => find(humn_n, e2, t, target / v),
            },
            Expr::Div(e1, e2) => match dfs1(humn_n, e1, t) {
                None => find(humn_n, e1, t, target * dfs(e2, t)),
                Some(v) => find(humn_n, e2, t, v / target),
            },
        }
    }
}

pub fn part1(input: &str) {
    let t = parse_input(input);
    let root_n = name_to_index(&mut "root".chars());
    let humn_n = name_to_index(&mut "humn".chars());
    match t[&root_n] {
        Expr::Lit(_) => panic!("root is lit"),
        Expr::Add(e1, e2) | Expr::Sub(e1, e2) | Expr::Mul(e1, e2) | Expr::Div(e1, e2) => {
            let (target, humn_e) = match dfs1(humn_n, e1, &t) {
                None => (dfs1(humn_n, e2, &t).unwrap(), e1),
                Some(t) => (t, e2),
            };
            let humn_v = find(humn_n, humn_e, &t, target);
            println!("{}", humn_v);
        }
    }
}

fn name_to_index<I>(i: &mut I) -> usize
where
    I: Iterator<Item = char>,
{
    fn num(c: char) -> usize {
        c as usize - 'a' as usize
    }
    num(i.next().unwrap())
        + num(i.next().unwrap()) * N
        + num(i.next().unwrap()) * N.pow(2)
        + num(i.next().unwrap()) * N.pow(3)
}

fn parse_input(input: &str) -> HashMap<usize, Expr> {
    fn parse_line(line: &str) -> (usize, Expr) {
        let mut iter = line.split(':');
        let label = name_to_index(&mut iter.next().unwrap().chars());
        let expr = iter.next().unwrap().trim();
        // dbg!(expr);
        match expr.parse::<i64>() {
            Ok(n) => (label, Expr::Lit(n)),
            Err(_) => {
                let mut iter = expr.split(' ');
                let var0 = name_to_index(&mut iter.next().unwrap().chars());
                let op = match iter.next().unwrap().chars().next().unwrap() {
                    '+' => Expr::Add,
                    '-' => Expr::Sub,
                    '*' => Expr::Mul,
                    '/' => Expr::Div,
                    _ => panic!("Unknown op"),
                };
                let var1 = name_to_index(&mut iter.next().unwrap().chars());
                (label, op(var0, var1))
            }
        }
    }
    let mut t = HashMap::with_capacity(5000);
    for (label, expr) in input.lines().map(parse_line) {
        t.insert(label, expr);
    }
    t
}
pub fn example_input() -> &'static str {
    r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"#
}
