use std::fmt;
use std::fs::File;
use std::io::Write;

use learning_graph::models::{Link, Place, TravelMode};

use learning_graph::startup::deserialization::{read_links, read_places};
use learning_graph::startup::graph_builder::{self};
use learning_graph::utils::dot::Dot;

//Run dot -Kfdp -n  example.dot -Tpng -o image.png
// or dot example.dot -Tpng -o image.png
fn main() {
    let places_path = "Places.csv";
    let links_path = "Links.csv";

    let nodes = read_places(places_path);
    let links = read_links(links_path);

    let (graph, _) = graph_builder::build(nodes, links);

    let dot = Dot::new(graph.clone(), &fmt_place, &fmt_link, &[&fmt_legend]);
    println!("{}", dot);

    let mut f = File::create("example.dot").unwrap();
    let output = format!("{}", dot);
    f.write_all(&output.as_bytes())
        .expect("could not write file");
}
fn fmt_place(f: &mut fmt::Formatter, data: &Place) -> fmt::Result {
    write!(f, "label = \"{}\\n{}\" ", data.name, &data.id)
}

fn travel_to_color(mode: TravelMode) -> &'static str {
    match mode {
        TravelMode::Foot => "#772A53",
        TravelMode::Bike => "#E9C46A",
        TravelMode::Car => "#E76F51",
        TravelMode::Bus => "#2A9D8F",
        TravelMode::Ship => "#9D7892",
        TravelMode::Rail => "#073B74",
    }
}

fn fmt_link(f: &mut fmt::Formatter, data: &Link) -> fmt::Result {
    write!(
        f,
        "color = \"{}\" penwidth=\"8\" label=\"{}\"",
        travel_to_color(data.mode),
        data.mode
    )
}

fn fmt_legend(f: &mut fmt::Formatter) -> fmt::Result {
    let a = &[
        TravelMode::Foot,
        TravelMode::Bike,
        TravelMode::Car,
        TravelMode::Bus,
        TravelMode::Ship,
        TravelMode::Rail,
    ];

    write!(
        f,
        "subgraph cluster_01 {{\nlabel = \"Legend\";\nnode [shape=point]\n{{\nrank=same\n"
    )?;

    for mode in a {
        write!(
            f,
            "{}0 [style = invis];\n{}1 [style = invis];\n",
            mode, mode
        )?;
    }
    write!(f, "}}\n")?;

    for mode in a {
        write!(
            f,
            "{0}0 -- {0}1 [label={0} color=\"{1}\" penwidth=\"8\"] \n",
            mode,
            travel_to_color(*mode)
        )?;
    }
    write!(f, "}}\n")
}
