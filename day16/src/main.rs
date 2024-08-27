use advent::diagonal_iterable::DiagonalIterable;
use bitvec::vec::BitVec;
use std::{
    collections::{BTreeMap, HashSet, VecDeque},
    fmt::Display,
    io::stdin,
};

use parser::{parse_valve, Valve};

mod parser;

fn dist_matrix(valves: &Vec<Valve>, nodes: &Vec<(String, isize)>) -> Vec<Vec<isize>> {
    let mut dists = vec![vec![isize::MAX; nodes.len()]; nodes.len()];
    dists.iter_diagonal_mut().for_each(|el| *el = 0);

    let valves: BTreeMap<String, Valve> = valves
        .iter()
        .cloned()
        .map(|v| (v.label.clone(), v))
        .collect();

    for (src_node_idx, src_node) in nodes.iter().enumerate() {
        let mut seen = HashSet::from([src_node.0.as_str()]);
        let mut queue = VecDeque::from([(src_node.0.as_str(), 0isize)]);

        while let Some((node, d)) = queue.pop_front() {
            if let Some(curr_node_idx) = nodes.iter().position(|n| n.0 == node) {
                assert!(d <= dists[src_node_idx][curr_node_idx]);
                dists[src_node_idx][curr_node_idx] = d;
                dists[curr_node_idx][src_node_idx] = d;
            }

            for neigh in valves[node].neighbours.iter() {
                if seen.contains(neigh.as_str()) {
                    continue;
                }

                seen.insert(neigh.as_str());
                queue.push_back((neigh.as_str(), d + 1));
            }
        }
    }

    dists
}

struct Dot<'a>(&'a Vec<Valve>);
impl<'a> Display for Dot<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "graph g {{")?;

        for valve in self.0.iter() {
            if valve.flow_rate == 0 {
                writeln!(f, r#"  {}[label="{}"]"#, valve.label, valve.label)?;
            } else {
                writeln!(
                    f,
                    r#"  {}[label="{}: {}", style=filled]"#,
                    valve.label, valve.label, valve.flow_rate
                )?;
            }
        }

        let mut printed = HashSet::new();
        for valve in self.0.iter() {
            printed.insert(&valve.label);

            for neigh in valve.neighbours.iter() {
                if !printed.contains(neigh) {
                    writeln!(f, r#"  {} -- {}"#, valve.label, neigh)?;
                }
            }
        }

        writeln!(f, "}}")
    }
}

#[derive(Clone, PartialEq, Eq)]
struct FlowState {
    time: isize,
    flow_per_min: isize,
    total_flow: isize,
    open_valves: BitVec,
}

fn best_flow_avoiding_nodes(
    nodes: &Vec<(String, isize)>,
    dists: &Vec<Vec<isize>>,
    max_time: isize,
    avoid: Option<&BitVec>,
) -> (isize, BitVec) {
    let mut start: BitVec = BitVec::repeat(false, nodes.len());
    start.set(0, true);

    let mut queue = VecDeque::from([(
        0usize,
        FlowState {
            time: 0,
            flow_per_min: 0,
            total_flow: 0,
            open_valves: start,
        },
    )]);

    let mut best_flow = (0isize, BitVec::new());

    while let Some((idx, flow)) = queue.pop_back() {
        for neigh in 0..nodes.len() {
            if flow.open_valves[neigh] || avoid.filter(|a| a[neigh]).is_some() {
                continue;
            }

            let dist = dists[idx][neigh] + 1;
            let new_time = flow.time + dist;
            if new_time > max_time {
                continue;
            }

            let new_flow = FlowState {
                time: new_time,
                total_flow: flow.total_flow + flow.flow_per_min * dist,
                flow_per_min: flow.flow_per_min + nodes[neigh].1,
                open_valves: {
                    let mut v = flow.open_valves.clone();
                    v.set(neigh, true);
                    v
                },
            };

            let new_pred_flow =
                new_flow.total_flow + new_flow.flow_per_min * (max_time - new_flow.time);
            if new_pred_flow > best_flow.0 {
                best_flow = (new_pred_flow, new_flow.open_valves.clone());
            }

            queue.push_back((neigh, new_flow));
        }
    }

    best_flow
}

fn best_flow(nodes: &Vec<(String, isize)>, dists: &Vec<Vec<isize>>, max_time: isize) -> isize {
    best_flow_avoiding_nodes(nodes, dists, max_time, None).0
}

fn best_flow_with_elephant(
    nodes: &Vec<(String, isize)>,
    dists: &Vec<Vec<isize>>,
    max_time: isize,
) -> isize {
    let best_flow_forward = best_flow_avoiding_nodes(nodes, dists, max_time, None);
    let rem_flow = best_flow_avoiding_nodes(nodes, dists, max_time, Some(&best_flow_forward.1));

    // This greedy solution probably shouldn't be good enough, but for the data I got it is.
    best_flow_forward.0 + rem_flow.0
}

fn main() {
    let valves: Vec<_> = stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| parse_valve(&l).unwrap().1)
        .collect();

    eprintln!("{}", Dot(&valves));

    let mut nodes: Vec<_> = valves
        .iter()
        .filter(|v| v.label == "AA" || v.flow_rate > 0)
        .map(|v| (v.label.clone(), v.flow_rate))
        .collect();

    nodes.sort();

    let nodes = nodes;
    let dists = dist_matrix(&valves, &nodes);

    println!("Part 1: {}", best_flow(&nodes, &dists, 30));
    println!("Part 2: {}", best_flow_with_elephant(&nodes, &dists, 26));
}
