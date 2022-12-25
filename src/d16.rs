use std::collections::{HashMap, VecDeque};

type AdjMat<T> = Vec<Vec<T>>;

fn parse_valve(input: &str) -> usize {
    // index two upper case chars
    let vs: Vec<usize> = input
        .chars()
        .take(2)
        .map(|c| c as usize - 'A' as usize)
        .collect();
    vs[0] * 26 + vs[1]
}

fn parse_line(input: &str) -> (usize, u32, Vec<usize>) {
    let mut iter = input.split("; ");
    let valve_part = iter.next().unwrap();
    let tunnel_part = iter.next().unwrap();
    let mut iter = valve_part.split('=');
    let from = parse_valve(iter.next().unwrap().strip_prefix("Valve ").unwrap());
    let flow_rate: u32 = iter.next().unwrap().parse().unwrap();
    let tos = tunnel_part.split(' ').skip(4).map(parse_valve).collect();
    (from, flow_rate, tos)
}

fn create_order_map(xs: &[usize]) -> (HashMap<usize, usize>, Vec<usize>) {
    let mut indexed: Vec<usize> = xs.iter().copied().collect();
    indexed.sort_unstable();
    indexed
        .iter()
        .enumerate()
        .map(|(i, x)| ((*x, i), *x))
        .unzip()
}

fn create_graph(
    froms: &Vec<usize>,
    toss: &[Vec<usize>],
) -> (AdjMat<Option<u32>>, HashMap<usize, usize>, Vec<usize>) {
    let n = froms.len();
    let mut graph = vec![vec![None; n]; n];
    let (v2o, o2v) = create_order_map(froms);

    // for connected valves, takes 1 min to go
    for (from, tos) in froms.iter().zip(toss.iter()) {
        let from = v2o[from];
        for to in tos.iter() {
            let to = v2o[to];
            graph[from][to] = Some(1);
        }
    }

    // diagonal
    (0..n).for_each(|i| graph[i][i] = Some(0));
    (graph, v2o, o2v)
}

/// pair-wise shortest path
fn floyd_warshall(graph: &mut AdjMat<Option<u32>>) {
    let n = graph.len();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                match graph[i][j] {
                    None => {
                        if let (Some(d_ik), Some(d_kj)) = (graph[i][k], graph[k][j]) {
                            graph[i][j] = Some(d_ik + d_kj)
                        }
                    }
                    Some(d_ij) => {
                        if let (Some(d_ik), Some(d_kj)) = (graph[i][k], graph[k][j]) {
                            let d_ikj = d_ik + d_kj;
                            if d_ikj < d_ij {
                                graph[i][j] = Some(d_ikj)
                            }
                        }
                    }
                }
            }
        }
    }
}

fn filter_non_zero(froms: &[usize], flows: &[u32]) -> Vec<usize> {
    froms
        .iter()
        .zip(flows.iter())
        .filter_map(|(v, f)| if *f != 0 { Some(*v) } else { None })
        .collect()
}

fn compress_graph(
    graph: &AdjMat<Option<u32>>,
    vs: &[usize],
) -> (AdjMat<Option<u32>>, HashMap<usize, usize>, Vec<usize>) {
    let (v2o, o2v) = create_order_map(vs);
    let n = vs.len();
    let mut new_graph = vec![vec![None; n]; n];
    for i in 0..n {
        for j in 0..n {
            new_graph[i][j] = graph[o2v[i]][o2v[j]];
        }
    }
    (new_graph, v2o, o2v)
}

struct State {
    at: usize,
    visited: Vec<bool>,
    minutes_left: u32,
    score: u32,
}

fn dfs(
    graph: &AdjMat<Option<u32>>,
    flows: &[u32],
    starts: &[(usize, u32)],
    total_minutes: u32,
) -> u32 {
    let mut queue: VecDeque<State> = VecDeque::new();
    let n = graph.len();
    for (start, minutes) in starts {
        let mut visited = vec![false; n];
        visited[*start] = true;
        let minutes_left = total_minutes - minutes - 1;
        let score = flows[*start] * minutes_left;
        queue.push_back(State {
            at: *start,
            visited,
            minutes_left,
            score,
        })
    }
    let mut best: u32 = 0;
    while !queue.is_empty() {
        let cur = queue.pop_front().unwrap();
        if cur.score > best {
            best = cur.score;
        }
        let nexts = graph[cur.at]
            .iter()
            .enumerate()
            .filter_map(|(i, x)| x.map(|x| (i, x)))
            .filter(|(i, x)| *x < cur.minutes_left && !cur.visited[*i]);
        for (next, minutes) in nexts {
            let mut visited = cur.visited.clone();
            visited[next] = true;
            let minutes_left = cur.minutes_left - minutes - 1;
            let next_score = flows[next] * minutes_left;
            queue.push_back(State {
                at: next,
                visited,
                minutes_left,
                score: cur.score + next_score,
            })
        }
    }
    best
}

pub fn part0(input: &str) {
    let (froms, (flows, toss)): (Vec<_>, (Vec<_>, Vec<_>)) = input
        .lines()
        .map(parse_line)
        .map(|(v, f, ts)| (v, (f, ts)))
        .unzip();
    // dbg!(&froms);
    // dbg!(&froms.iter().zip(flows.iter()).collect::<Vec<_>>());
    // dbg!(&toss);
    let (mut graph, name2graph, graph2name) = create_graph(&froms, &toss);
    // dbg!(&graph);
    floyd_warshall(&mut graph);
    // dbg!(&graph);
    let non_zeros: Vec<usize> = filter_non_zero(&froms, &flows)
        .iter()
        .map(|i| name2graph[i])
        .collect();
    // dbg!(&non_zeros);
    let (compressed_graph, graph2cgraph, cgraph2graph) = compress_graph(&graph, &non_zeros);
    // dbg!(&graph);
    let starts: Vec<(usize, u32)> = graph[0]
        .iter()
        .enumerate()
        .filter_map(|(i, x)| {
            if let (Some(i), Some(x)) = (graph2cgraph.get(&i), x) {
                Some((*i, *x))
            } else {
                None
            }
        })
        .filter(|(_i, x)| *x < 30)
        .collect();
    // dbg!(&compress_order_map);
    // dbg!(&starts);
    let name2flow: HashMap<usize, u32> = froms
        .iter()
        .zip(flows.iter())
        .map(|(from, flow)| (*from, *flow))
        .collect();
    let compressed_flows: Vec<u32> = (0..compressed_graph.len())
        .map(|i| name2flow[&graph2name[cgraph2graph[i]]])
        .collect();
    let ans = dfs(&compressed_graph, &compressed_flows, &starts, 30);
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
