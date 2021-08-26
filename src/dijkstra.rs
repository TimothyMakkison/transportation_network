use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::graph::Node;
use crate::graph::{EdgeRef, Graph, NodeIndex};
use crate::scored::MinScored;

pub fn dijkstra<N, E, T, S>(
    graph: Graph<N, E>,
    start: NodeIndex,
    goal: Option<NodeIndex>,
    get_cost: S,
    traversable: T,
) -> HashMap<NodeIndex, i32>
where
    S: Fn(&E) -> i32,
    T: Fn(&E) -> bool,
{
    let mut unvisited_queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut scores = HashMap::new();

    unvisited_queue.push(MinScored(0, start));
    scores.insert(start, 0);

    while let Some(MinScored(score, node_id)) = unvisited_queue.pop() {
        if goal == Some(node_id) {
            break;
        }
        if (visited.contains(&node_id)) {
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
