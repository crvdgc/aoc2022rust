use std::cell::RefCell;
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

fn eval(t: &HashMap<usize, RefCell<Expr>>) {
    for expr in t.values() {
        expr.replace_with(|&mut old| match old {
            Expr::Add(e0, e1) => {
                if let (Expr::Lit(v0), Expr::Lit(v1)) = (*t[&e0].borrow(), *t[&e1].borrow()) {
                    Expr::Lit(v0 + v1)
                } else {
                    old
                }
            }
            Expr::Sub(e0, e1) => {
                if let (Expr::Lit(v0), Expr::Lit(v1)) = (*t[&e0].borrow(), *t[&e1].borrow()) {
                    Expr::Lit(v0 - v1)
                } else {
                    old
                }
            }
            Expr::Mul(e0, e1) => {
                if let (Expr::Lit(v0), Expr::Lit(v1)) = (*t[&e0].borrow(), *t[&e1].borrow()) {
                    Expr::Lit(v0 * v1)
                } else {
                    old
                }
            }
            Expr::Div(e0, e1) => {
                if let (Expr::Lit(v0), Expr::Lit(v1)) = (*t[&e0].borrow(), *t[&e1].borrow()) {
                    Expr::Lit(v0 / v1)
                } else {
                    old
                }
            }
            Expr::Lit(_) => old,
        });
    }
}

pub fn part0(input: &str) {
    let t = parse_input(input);
    // dbg!(&t);
    for _ in 0..t.len() {
        if let Some(r) = t.get(&name_to_index(&mut "root".chars())) {
            // dbg!("has name");
            if let Expr::Lit(n) = *r.borrow() {
                println!("{}", n);
                break;
            }
        }
        eval(&t);
    }
    // dbg!(&t);
}

pub fn part1(input: &str) {}

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

fn parse_input(input: &str) -> HashMap<usize, RefCell<Expr>> {
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
        t.insert(label, RefCell::new(expr));
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
