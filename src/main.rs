use std::collections::HashMap;

use learning_graph::process_command::CommandProcessor;
use learning_graph::startup::deserialization::{read_commands, read_links, read_places};
use learning_graph::startup::graph_builder::{self};

fn main() {
    let places_path = "Places.csv";
    let links_path = "Links.csv";
    let command_path = "Commands.txt";

    let nodes = read_places(places_path);
    let links = read_links(links_path);

    let mut counter = HashMap::new();
    for link in links.clone() {
        counter
            .entry(link.mode)
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    println!("{:?}", counter);

    let (graph, map) = graph_builder::build(nodes, links);

    let processor = CommandProcessor::new(graph, map);

    let commands = read_commands(command_path).unwrap();

    let results: Vec<String> = commands.into_iter().map(|x| processor.process(x)).collect();

    for result in results {
        println!("{}\n", result);
    }
}
