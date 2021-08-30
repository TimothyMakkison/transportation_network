extern crate learning_graph;

use learning_graph::{dijkstra::dijkstra, graph::Graph};

fn main() {
    let mut graph = Graph::new();

    let node_a = graph.add_node("a");
    let node_b = graph.add_node("b");
    let node_c = graph.add_node("c");
    let node_d = graph.add_node("d");
    let node_e = graph.add_node("e");

    graph.add_edge(node_a, node_d, 1);
    graph.add_edge(node_a, node_b, 6);

    graph.add_edge(node_d, node_b, 2);
    graph.add_edge(node_d, node_e, 1);

    graph.add_edge(node_e, node_b, 2);
    graph.add_edge(node_e, node_c, 5);

    graph.add_edge(node_b, node_c, 5);

    let result = dijkstra(&graph, node_a, None, |x| *x, |_| true);

    println!("{:?}", result);
}

#[test]
fn test_disktra() {
    let mut graph = Graph::new();

    let node_a = graph.add_node("a");
    let node_b = graph.add_node("b");
    let node_c = graph.add_node("c");
    let node_d = graph.add_node("d");
    let node_e = graph.add_node("e");

    graph.add_edge(node_a, node_d, 1);
    graph.add_edge(node_a, node_b, 6);

    graph.add_edge(node_d, node_b, 2);
    graph.add_edge(node_d, node_e, 1);

    graph.add_edge(node_e, node_b, 2);
    graph.add_edge(node_e, node_c, 5);

    graph.add_edge(node_b, node_c, 5);

    let result = dijkstra(&graph, node_a, None, |x| *x, |_| true);

    let expected = [
        (0, (0, 0)),
        (1, (3, 3)),
        (2, (7, 4)),
        (3, (1, 0)),
        (4, (2, 3)),
    ]
    .iter()
    .cloned()
    .collect();

    assert_eq!(result, expected);
}
