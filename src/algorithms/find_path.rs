use std::collections::{BinaryHeap, HashMap, HashSet};

// use crate::graph::{EdgeRef, Graph, NodeIndex};

pub fn find_path<N, E, T, TGetCost, TCost>(
    graph: &Graph<N, E>,
    start: NodeIndex,
    goal: NodeIndex,
    get_cost: TGetCost,
    traversable: T,
) -> HashMap<NodeIndex, (TCost, NodeIndex)>
where
    TGetCost: Fn(&N, &N) -> TCost,
    T: Fn(&E) -> bool,
    TCost: Default + PartialOrd + Default + Clone + Copy,
{
    let mut unvisited_queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut scores = HashMap::new();

    let default_score = TCost::default();

    unvisited_queue.push(MinScored(default_score, start));
    scores.insert(start, (default_score, start));

    while let Some(MinScored(_, node_id)) = unvisited_queue.pop() {
        if visited.contains(&node_id) {
            continue;
        }
        if node_id == goal {
            break;
        }

        visited.insert(node_id);

        for edge_ref in graph
            .edges(node_id)
            .filter(|x| traversable(x.data))
            .filter(|edge_ref| !visited.contains(&edge_ref.destination()))
        {
            let source_id = edge_ref.source();
            let dest_id = edge_ref.destination();

            let source_node = graph.get_node(source_id).unwrap();
            let dest_node = graph.get_node(dest_id).unwrap();

            let cost = get_cost(&source_node.data, &dest_node.data);

            scores.entry(dest_id).or_insert({
                unvisited_queue.push(MinScored(cost, dest_id));
                (cost, node_id)
            });
        }
    }

    scores
}
use std::cmp::Ordering;

use super::graph::{EdgeRef, Graph, NodeIndex};

/// `MinScored<K, T>` holds a score `K` and a scored object `T` in
/// a pair for use with a `BinaryHeap`.
///
/// `MinScored` compares in reverse order by the score, so that we can
/// use `BinaryHeap` as a min-heap to extract the score-value pair with the
/// least score.
///
/// **Note:** `MinScored` implements a total order (`Ord`), so that it is
/// possible to use float types as scores.
#[derive(Copy, Clone, Debug)]
pub struct MinScored<TScore, TNode>(pub TScore, pub TNode);

impl<K: PartialOrd, T> PartialEq for MinScored<K, T> {
    #[inline]
    fn eq(&self, other: &MinScored<K, T>) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<K: PartialOrd, T> Eq for MinScored<K, T> {}

impl<K: PartialOrd, T> PartialOrd for MinScored<K, T> {
    #[inline]
    fn partial_cmp(&self, other: &MinScored<K, T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: PartialOrd, T> Ord for MinScored<K, T> {
    #[inline]
    fn cmp(&self, other: &MinScored<K, T>) -> Ordering {
        let a = &self.0;
        let b = &other.0;
        if a == b {
            Ordering::Equal
        } else if a < b {
            Ordering::Greater
        } else if a > b {
            Ordering::Less
        } else if a.ne(a) && b.ne(b) {
            // these are the NaN cases
            Ordering::Equal
        } else if a.ne(a) {
            // Order NaN less, so that it is last in the MinScore order
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}
