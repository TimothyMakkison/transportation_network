use criterion::{black_box, criterion_group, criterion_main, Criterion};

use learning_graph::algorithms::graph::{Graph, NodeIndex};
use learning_graph::models::Link;
use learning_graph::models::Place;
use learning_graph::process_command::CommandProcessor;
use learning_graph::startup::deserialization::{read_commands, read_links, read_places};

use std::collections::HashMap;

fn run() {
    let places_path = "Places.csv";
    let links_path = "Links.csv";
    let command_path = "Commands.txt";

    let nodes = read_places(places_path);
    let links = read_links(links_path);

    let (graph, map) = build_graph(nodes, links);

    let processor = CommandProcessor::new(graph, map);

    let commands = read_commands(command_path).unwrap();

    let results: Vec<String> = commands.into_iter().map(|x| processor.process(x)).collect();

    for result in results {
        println!("{}\n", result);
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

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("run 20", |b| b.iter(|| run()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
