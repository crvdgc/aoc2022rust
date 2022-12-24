use std::collections::{HashMap, HashSet};

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

fn create_order_map(xs: &[usize], index_first: bool) -> HashMap<usize, usize> {
    let mut indexed: Vec<(usize, usize)> = xs.iter().copied().enumerate().collect();
    indexed.sort_by_key(|(_i, x)| *x);
    indexed
        .iter()
        .map(|(i, x)| if index_first { (*i, *x) } else { (*x, *i) })
        .collect()
}

fn create_graph(
    froms: &Vec<usize>,
    toss: &[Vec<usize>],
) -> (AdjMat<Option<u32>>, HashMap<usize, usize>) {
    let n = froms.len();
    let mut graph = vec![vec![None; n]; n];
    let rev_order_map = create_order_map(froms, false);

    // for connected valves, takes 1 min to go
    for (from, tos) in froms.iter().zip(toss.iter()) {
        let from = rev_order_map.get(from).unwrap();
        for to in tos.iter() {
            let to = rev_order_map.get(to).unwrap();
            graph[*from][*to] = Some(1);
        }
    }

    // diagonal
    (0..n).for_each(|i| graph[i][i] = Some(0));
    (graph, rev_order_map)
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

fn compress_graph(graph: &AdjMat<Option<u32>>, vs: &[usize]) -> AdjMat<Option<u32>> {
    let order_map = create_order_map(vs, true);
    let n = order_map.len();
    let mut new_graph = vec![vec![None; n]; n];
    for i in 0..n {
        for j in 0..n {
            new_graph[i][j] = graph[*order_map.get(&i).unwrap()][*order_map.get(&j).unwrap()];
        }
    }
    new_graph
}

pub fn part0(input: &str) {
    let (froms, (flows, toss)): (Vec<_>, (Vec<_>, Vec<_>)) = input
        .lines()
        .map(parse_line)
        .map(|(v, f, ts)| (v, (f, ts)))
        .unzip();
    // dbg!(&froms);
    // dbg!(&flows);
    // dbg!(&toss);
    let (mut graph, rev_order_map) = create_graph(&froms, &toss);
    // dbg!(&graph);
    floyd_warshall(&mut graph);
    // dbg!(&graph);
    let non_zeros: Vec<usize> = filter_non_zero(&froms, &flows)
        .iter()
        .map(|x| *rev_order_map.get(x).unwrap())
        .collect();
    let graph = compress_graph(&graph, &non_zeros);
    dbg!(&graph);
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
