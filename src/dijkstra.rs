use std::collections::{BinaryHeap, HashMap, HashSet};
use std::ops::Add;

use crate::graph::{EdgeRef, Graph, NodeIndex};

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
use std::cmp::Ordering;

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
pub struct MinScored<K, T>(pub K, pub T);

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
