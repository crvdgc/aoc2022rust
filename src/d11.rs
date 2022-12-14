use crate::util;
use std::collections::VecDeque;
use std::fmt;

enum Op {
    Add(Option<i64>),
    Prod(Option<i64>),
}

impl Op {
    fn eval(&self, n: i64) -> i64 {
        match &self {
            Op::Add(None) => n + n,
            Op::Add(Some(i)) => n + i,
            Op::Prod(None) => n * n,
            Op::Prod(Some(i)) => n * i,
        }
    }
}

impl fmt::Debug for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Op::Add(Some(i)) => write!(f, "+ {}", i),
            Op::Add(None) => write!(f, "+ old"),
            Op::Prod(Some(i)) => write!(f, "* {}", i),
            Op::Prod(None) => write!(f, "* old"),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<i64>,
    op: Op,
    divisible: i64,
    to_true: usize,
    to_false: usize,
}

impl Monkey {
    fn of_line(input: &str) -> Self {
        let mut lines = input.lines().map(|x| x.trim());
        let _ = lines.next(); // ignore Monkey n line
        let items: VecDeque<i64> = util::next_line_prefixed(&mut lines, "Starting items: ")
            .split(", ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        let mut op_iter = util::next_line_prefixed(&mut lines, "Operation: new = old ").split(' ');
        let op_f = match op_iter.next().unwrap() {
            "+" => Op::Add,
            "*" => Op::Prod,
            op => panic!("Unknown op {}", op),
        };
        let op_n: Option<i64> = op_iter.next().unwrap().parse().ok();
        let op = op_f(op_n);

        let divisible: i64 = util::next_line_prefixed(&mut lines, "Test: divisible by ")
            .parse()
            .unwrap();

        let to_true: usize = util::next_line_prefixed(&mut lines, "If true: throw to monkey ")
            .parse()
            .unwrap();
        let to_false: usize = util::next_line_prefixed(&mut lines, "If false: throw to monkey ")
            .parse()
            .unwrap();

        Monkey {
            items,
            op,
            divisible,
            to_true,
            to_false,
        }
    }

    fn throw_items(&mut self) -> Vec<(usize, i64)> {
        let mut thrown = Vec::with_capacity(self.items.len());
        loop {
            match self.items.pop_front() {
                None => break,
                Some(item) => {
                    let item = self.op.eval(item) / 3;
                    let to = if item % self.divisible == 0 {
                        self.to_true
                    } else {
                        self.to_false
                    };
                    thrown.push((to, item));
                }
            }
        }
        thrown
    }
    fn throw_items_part1(&mut self, modulo_group: i64) -> Vec<(usize, i64)> {
        let mut thrown = Vec::with_capacity(self.items.len());
        loop {
            match self.items.pop_front() {
                None => break,
                Some(item) => {
                    let item = self.op.eval(item) % modulo_group;
                    let to = if item % self.divisible == 0 {
                        self.to_true
                    } else {
                        self.to_false
                    };
                    thrown.push((to, item));
                }
            }
        }
        thrown
    }
}

fn run_round(monkeys: &mut Vec<Monkey>, inspection: &mut [u64]) {
    let n = monkeys.len();
    for from in 0..n {
        for (to, item) in monkeys[from].throw_items() {
            inspection[from] += 1;
            monkeys[to].items.push_back(item)
        }
    }
}

pub fn part0(input: &str) {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::of_line).collect();
    let mut inspection: Vec<u64> = vec![0; monkeys.len()];
    for _ in 0..20 {
        run_round(&mut monkeys, &mut inspection);
    }
    // dbg!(&inspection);
    inspection.sort_unstable();
    let monkey_business: u64 = inspection.iter().rev().take(2).product();
    println!("{}", monkey_business);
}

fn run_round_part1(monkeys: &mut Vec<Monkey>, inspection: &mut [u64], modulo_group: i64) {
    let n = monkeys.len();
    for from in 0..n {
        for (to, item) in monkeys[from].throw_items_part1(modulo_group) {
            inspection[from] += 1;
            monkeys[to].items.push_back(item)
        }
    }
}

pub fn part1(input: &str) {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::of_line).collect();
    let mut inspection: Vec<u64> = vec![0; monkeys.len()];
    let modulo_group: i64 = monkeys.iter().map(|x| x.divisible).product();
    for _ in 0..10000 {
        run_round_part1(&mut monkeys, &mut inspection, modulo_group);
    }
    dbg!(&inspection);
    inspection.sort_unstable();
    let monkey_business: u64 = inspection.iter().rev().take(2).product();
    println!("{}", monkey_business);
}

pub fn example_input() -> &'static str {
    r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#
}
