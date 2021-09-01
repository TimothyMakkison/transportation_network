use learning_graph::algorithms::graph::Graph;

extern crate learning_graph;

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

    for i in graph.edges(node_b) {
        println!("{:?}", i);
    }
}
