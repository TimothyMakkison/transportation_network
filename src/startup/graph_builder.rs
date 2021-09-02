use std::collections::HashMap;

use crate::{
    algorithms::graph::{Graph, NodeIndex},
    models::{Link, Place},
};

pub fn build(nodes: Vec<Place>, edges: Vec<Link>) -> (Graph<Place, Link>, HashMap<i32, NodeIndex>) {
    let mut graph = Graph::new();
    let mut map = HashMap::new();

    for node in nodes {
        let id = node.id;
        let node_index = graph.add_node(node);
        map.insert(id, node_index);
    }

    for edge in edges {
        let a = map.get(&edge.start).unwrap();
        let b = map.get(&edge.end).unwrap();
        graph.add_edge(*a, *b, edge);
    }

    (graph, map)
}
