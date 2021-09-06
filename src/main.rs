use std::fmt;
use std::fs::File;
use std::io::Write;

use learning_graph::display::Dot;
use learning_graph::models::{Link, Place, TravelMode};
use learning_graph::process_command::CommandProcessor;
use learning_graph::startup::deserialization::{read_commands, read_links, read_places};
use learning_graph::startup::graph_builder::{self};

fn main() {
    let places_path = "Places.csv";
    let links_path = "Links.csv";
    let command_path = "Commands.txt";

    let nodes = read_places(places_path);
    let links = read_links(links_path);

    let (graph, map) = graph_builder::build(nodes, links);

    let dot = Dot::new(graph.clone(), &fmt_place, &fmt_link, &[]);
    println!("{}", dot);

    let mut f = File::create("example.dot").unwrap();
    let output = format!("{}", dot);
    f.write_all(&output.as_bytes())
        .expect("could not write file");

    let processor = CommandProcessor::new(graph, map);

    let commands = read_commands(command_path).unwrap();

    let results: Vec<String> = commands.into_iter().map(|x| processor.process(x)).collect();

    for result in results {
        // println!("{}\n", result);
    }
}

fn fmt_place(f: &mut fmt::Formatter, data: &Place) -> fmt::Result {
    write!(f, "label = \"{}\\n{}\" ", data.name, &data.id)
}

fn travel_to_color(mode: TravelMode) -> &'static str {
    match mode {
        TravelMode::Foot => "#FF9AA2",
        TravelMode::Bike => "#FFB7B2",
        TravelMode::Car => "#F4C87E",
        TravelMode::Bus => "#E2F0CB",
        TravelMode::Ship => "#A0CDCA", //Cyan
        TravelMode::Rail => "#617E96", //Navy
    }
}

fn fmt_link(f: &mut fmt::Formatter, data: &Link) -> fmt::Result {
    write!(
        f,
        "color = \"{}\" penwidth=\"8\"",
        travel_to_color(data.mode)
    )
}
