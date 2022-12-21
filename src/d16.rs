use std::collections::{HashMap, HashSet};

type Valve = (char, char);
type AdjEdge = HashMap<Valve, HashSet<Valve>>;

fn insert_edge(graph: &mut AdjEdge, from: Valve, to: Valve) {
    match graph.get_mut(&from) {
        None => {
            graph.insert(from, HashSet::from([to]));
        }
        Some(edges) => {
            edges.insert(to);
        }
    }
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
    let from = parse_valve(iter.next().unwrap().strip_prefix("Valve ").unwrap());
    let flow_rate: u32 = iter.next().unwrap().parse().unwrap();
    let tos = tunnel_part.split(' ').skip(4).map(parse_valve).collect();
    (from, flow_rate, tos)
}

#[derive(Debug)]
struct ValveState {
    flow_rate: u32,
    opened: bool,
}

impl ValveState {
    fn new(flow_rate: u32) -> Self {
        Self {
            flow_rate,
            opened: false,
        }
    }
}

fn init_state(graph: &AdjEdge, flow_rates: &HashMap<Valve, u32>) -> HashMap<Valve, ValveState> {
    graph
        .keys()
        .map(|k| (*k, ValveState::new(*flow_rates.get(k).unwrap())))
        .collect()
}

fn dfs(
    state: &mut HashMap<Valve, ValveState>,
    graph: &AdjEdge,
    at: Valve,
    minutes_left: u32,
    score: u32,
) -> u32 {
    if minutes_left == 0 {
        score
    } else {
        let cur = state.get_mut(&at).unwrap();
        let cur_flow_rate = cur.flow_rate;
        let mut candidates: Vec<u32> = Vec::new();
        if !cur.opened && cur_flow_rate > 0 {
            cur.opened = true;
            let minutes_left = minutes_left - 1;
            let rest_best = dfs(
                state,
                graph,
                at,
                minutes_left,
                score + cur_flow_rate * minutes_left,
            );
            candidates.push(rest_best);
            state.get_mut(&at).unwrap().opened = false;
        }
        for neighbor in graph.get(&at).unwrap() {
            let rest_best = dfs(state, graph, *neighbor, minutes_left - 1, score);
            candidates.push(rest_best);
        }
        *candidates.iter().max().unwrap()
    }
}

pub fn part0(input: &str) {
    let (valves, (flows, toss)): (Vec<_>, (Vec<_>, Vec<_>)) = input
        .lines()
        .map(parse_line)
        .map(|(v, f, ts)| (v, (f, ts)))
        .unzip();
    // dbg!(&valves);
    let graph: AdjEdge = {
        let mut graph = HashMap::new();
        for (from, tos) in valves.iter().zip(toss.iter()) {
            for to in tos {
                insert_edge(&mut graph, *from, *to);
            }
        }
        graph
    };
    let flow_rates: HashMap<Valve, u32> = valves
        .iter()
        .zip(flows.iter())
        .map(|(v, f)| (*v, *f))
        .collect();
    let mut state = init_state(&graph, &flow_rates);
    // dbg!(&state);
    let ans = dfs(&mut state, &graph, ('A', 'A'), 10, 0);
    println!("{}", ans);
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
