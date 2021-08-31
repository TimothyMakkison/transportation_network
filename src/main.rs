use learning_graph::models::Link;
use learning_graph::models::Place;
use learning_graph::process_command::CommandProcessor;
use learning_graph::{
    deserialization::{read_commands, read_links, read_places},
    graph::{Graph, NodeIndex},
};

use std::collections::HashMap;

fn main() {
    let places_path = "Places.csv";
    let links_path = "Links.csv";
    let command_path = "Commands.txt";

    let nodes = read_places(places_path);
    let links = read_links(links_path);

    let (graph, map) = build_graph(nodes, links);

    let processor = CommandProcessor::new(graph, map);

    let commands = read_commands(command_path).unwrap();
    // println!("{:?}", commands);

    let results: Vec<String> = commands.into_iter().map(|x| processor.process(x)).collect();

    for result in results {
        println!("{}", result);
    }
}

fn build_graph(
    nodes: Vec<Place>,
    edges: Vec<Link>,
) -> (Graph<Place, Link>, HashMap<i32, NodeIndex>) {
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
