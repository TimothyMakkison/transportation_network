use learning_graph::process_command::CommandProcessor;
use learning_graph::startup::deserialization::{read_commands, read_links, read_places};
use learning_graph::startup::graph_builder::{self};

fn main() {
    const PLACES_PATH: &str = "Places.csv";
    const LINKS_PATH: &str = "Links.csv";
    const COMMAND_PATH: &str = "Commands.txt";

    let nodes = read_places(PLACES_PATH);
    let links = read_links(LINKS_PATH);

    let (graph, map) = graph_builder::build(nodes, links);

    let processor = CommandProcessor::new(graph, map);

    let commands = read_commands(COMMAND_PATH).unwrap();

    let results: Vec<String> = commands.into_iter().map(|x| processor.process(x)).collect();

    for result in results {
        println!("{}\n", result);
    }
}
