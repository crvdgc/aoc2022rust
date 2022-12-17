use std::cmp::Ordering;

#[derive(Eq, Clone)]
enum Packet {
    Number(u32),
    List(Vec<Packet>),
}

impl std::fmt::Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(arg0) => f.write_fmt(format_args!("{}", arg0)),
            Self::List(arg0) => f.debug_list().entries(arg0.iter()).finish(),
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Number(l0), Self::List(r0)) => vec![Self::Number(*l0)] == *r0,
            (Self::List(l0), Self::Number(r0)) => *l0 == vec![Self::Number(*r0)],
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0.partial_cmp(r0),
            (Self::List(l0), Self::List(r0)) => {
                let ll = l0.len();
                let lr = r0.len();
                let min_l = std::cmp::min(ll, lr);
                for i in 0..min_l {
                    match l0[i].partial_cmp(&r0[i]) {
                        Some(Ordering::Equal) => continue,
                        other => return other,
                    }
                }
                ll.partial_cmp(&lr)
            }
            (Self::Number(l0), Self::List(r0)) => vec![Self::Number(*l0)].partial_cmp(r0),
            (Self::List(l0), Self::Number(r0)) => (l0).partial_cmp(&vec![Self::Number(*r0)]),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse(input: &str, mut stack: Vec<Packet>) -> (&str, Vec<Packet>) {
    if let Some(c) = input.chars().next() {
        match c {
            '[' => {
                let inner_stack = Vec::new();
                let (rest, inner) = parse(&input[1..], inner_stack);
                stack.push(Packet::List(inner));
                parse(rest, stack)
            }
            ']' => (&input[1..], stack),
            ',' => parse(&input[1..], stack),
            _ => {
                let end = input.chars().position(|x| x == ',' || x == ']').unwrap();
                // dbg!(&input[..end]);
                let n: u32 = (&input[..end]).parse().unwrap();
                stack.push(Packet::Number(n));
                parse(&input[end..], stack)
            }
        }
    } else {
        (input, stack)
    }
}

fn parse_group(input: &str) -> (Packet, Packet) {
    let mut lines = input.lines();
    let p1 = lines.next().unwrap();
    let (_, packets1) = parse(p1, Vec::new());
    let p2 = lines.next().unwrap();
    let (_, packets2) = parse(p2, Vec::new());
    (Packet::List(packets1), Packet::List(packets2))
}

pub fn part0(input: &str) {
    let groups = input.split("\n\n");
    let ans: usize = groups
        .map(parse_group)
        .enumerate()
        .map(|(idx, (p1, p2))| if p1 < p2 { idx + 1 } else { 0 })
        .sum();
    println!("{}", ans);
}
pub fn part1(input: &str) {
    let mut packets: Vec<Packet> = input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| parse(x, Vec::new()))
        .map(|(_, packets)| Packet::List(packets))
        .collect();
    let d0 = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    let d1 = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);
    packets.push(d0.clone());
    packets.push(d1.clone());
    packets.sort();
    let idx0 = packets.iter().position(|x| *x == d0).unwrap();
    let idx1 = packets.iter().position(|x| *x == d1).unwrap();
    let ans: usize = (idx0 + 1) * (idx1 + 1);
    println!("{}", ans);
}
pub fn example_input() -> &'static str {
    r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#
}
