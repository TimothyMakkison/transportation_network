use std::collections::{BinaryHeap, HashMap, HashSet};
use std::ops::Add;

use crate::graph::{EdgeRef, Graph, NodeIndex};
use crate::scored::MinScored;

pub fn dijkstra<N, E, T, S, C>(
    graph: Graph<N, E>,
    start: NodeIndex,
    goal: Option<NodeIndex>,
    get_cost: S,
    traversable: T,
) -> HashMap<NodeIndex, C>
where
    S: Fn(&E) -> C,
    T: Fn(&E) -> bool,
    C: Default + Ord + PartialOrd + Add<C, Output = C> + Default + Clone + Copy,
{
    let mut unvisited_queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut scores = HashMap::new();

    let default_score = C::default();

    unvisited_queue.push(MinScored(default_score, start));
    scores.insert(start, default_score);

    while let Some(MinScored(score, node_id)) = unvisited_queue.pop() {
        if goal == Some(node_id) {
            break;
        }
        if visited.contains(&node_id) {
            continue;
        }

        visited.insert(node_id);

        for edge_ref in graph
            .edges(node_id)
            .filter(|x| traversable(x.data))
            .filter(|edge_ref| !visited.contains(&edge_ref.destination()))
        {
            let dest_id = edge_ref.destination();
            let cost = get_cost(edge_ref.data);
            let total_cost = cost + score;

            scores
                .entry(dest_id)
                .and_modify(|entry| {
                    if total_cost < *entry {
                        // Update score in priority queue
                        unvisited_queue.push(MinScored(total_cost, dest_id));
                        *entry = total_cost
                    }
                })
                .or_insert({
                    unvisited_queue.push(MinScored(total_cost, dest_id));
                    total_cost
                });
        }
    }

    scores
}
