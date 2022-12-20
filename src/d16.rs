use std::collections::{HashMap, HashSet};

type Valve = (char, char);
type RevAdjEdge = HashMap<Valve, HashSet<Valve>>;

fn insert_edge(rev_adj_edge: &mut RevAdjEdge, from: Valve, to: Valve) {
    match rev_adj_edge.get_mut(&to) {
        None => {
            rev_adj_edge.insert(to, HashSet::from([from]));
        }
        Some(edges) => {
            edges.insert(from);
        }
    }
}

fn init_scores(rev_adj_edge: &RevAdjEdge) -> HashMap<Valve, Option<u32>> {
    let mut scores: HashMap<Valve, Option<u32>> = rev_adj_edge.keys().map(|k| (*k, None)).collect();
    match scores.get_mut(&('A', 'A')) {
        Some(score) => *score = Some(0),
        None => {
            scores.insert(('A', 'A'), Some(0));
        }
    }
    scores
}

fn parse_valve(input: &str) -> Valve {
    let mut iter = input.chars();
    (iter.next().unwrap(), iter.next().unwrap())
}

fn parse_line(input: &str) -> (Valve, u32, Vec<Valve>) {
    let mut iter = input.split("; ");
    let valve_part = iter.next().unwrap();
    let tunnel_part = iter.next().unwrap();
    let mut iter = valve_part.split('=');
    let from = parse_valve(iter.next().unwrap());
    let flow_rate: u32 = iter.next().unwrap().parse().unwrap();
    let tos = tunnel_part.split(' ').skip(4).map(parse_valve).collect();
    (from, flow_rate, tos)
}

pub fn part0(input: &str) {
    let lines: Vec<(Valve, u32, Vec<Valve>)> = input.lines().map(parse_line).collect();
    dbg!(lines);
}

pub fn part1(input: &str) {}

pub fn example_input() -> &'static str {
    r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#
}
