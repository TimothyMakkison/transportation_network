use learning_graph::{
    dijkstra::dijkstra,
    graph::{Graph, NodeIndex},
    models::{Command, Link, Place, TravelMode},
    parser::parse_command,
    serialization::{read_commands, read_links, read_places},
};

use std::{collections::HashMap, fmt::Display, fs, hash::Hash};

fn main() {
    let places_path = "Places.csv";
    let links_path = "Links.csv";

    let nodes = read_places(places_path);
    let links = read_links(links_path);

    // let edges =
    let commands = read_commands("Commands.txt");
    println!("{:?}", commands);
}

// fn build_graph(
//     nodes: Vec<Place>,
//     edges: Vec<Link>,
// ) -> (Graph<Place, Link>, HashMap<i32, NodeIndex>) {
//     let mut graph = Graph::new();
//     let mut map = HashMap::new();

//     for node in nodes {
//         let boxed = Box::new(node);
//         let node_index = graph.add_node(node);
//         map.insert(node.id, node_index);
//     }

//     (graph, map)
// }

fn process_command<E, N>(
    graph: &Graph<E, N>,
    node_map: HashMap<i32, NodeIndex>,
    command: Command,
) -> String {
    match command {
        Command::MaxDist => "max dist".to_string(),
        Command::FindNeighbour(place) => place.to_string(),
        // Command::FindShortestRoute(mode,source,goal) => find_shortest_route(graph)
        _ => "Not impl".to_string(),
    }
}

fn find_shortest_route<N, E>(
    graph: &Graph<N, E>,
    id_map: HashMap<i32, NodeIndex>,
    command: Command,
) -> String
where
    N: Display + Copy,
{
    if let Command::FindShortestRoute(mode, start, goal) = command {
        let start_node = id_map.get(&start).unwrap();
        let goal_node = id_map.get(&start).unwrap();

        let mut output = format!("FindShortestRoute {} {} {}", mode, start, goal);
        let routes = dijkstra(graph, *start_node, Some(*goal_node), |_| 1, |x| true);

        let mut nodes = vec![goal_node];
        let mut curr = goal_node;

        while curr != start_node {
            match routes.get(curr) {
                Some(pair) => {
                    curr = &pair.1;
                    nodes.push(curr);
                }
                None => {
                    output.push_str("\n Fail");
                    return output;
                }
            }
        }
        nodes.reverse();

        for i in nodes {
            let node = graph.get_node(*i).unwrap();
            output.push_str(&node.data.to_string());
        }

        output.to_string()
    } else {
        panic!();
    }
}

//TODO Use flags, recursion or comparison???.

// 1. A rail or ship journey may only use Arcs of the corresponding mode;
// 2. A bus journey may use bus and ship Arcs, while a car journey may use car, bus and ship Arcs;
// 3. A bike journey may use bike Arcs and Arcs defined in 1 and 2;
// 4. A foot journey may use any Arc.

fn can_traverse(mode: &TravelMode, edge_mode: &TravelMode) -> bool {
    match mode {
        TravelMode::Rail => *edge_mode == TravelMode::Rail,
        TravelMode::Ship => *edge_mode == TravelMode::Ship,
        TravelMode::Bus => *edge_mode == TravelMode::Bus || *edge_mode == TravelMode::Ship,
        TravelMode::Car => {
            *edge_mode == TravelMode::Car
                || *edge_mode == TravelMode::Bus
                || *edge_mode == TravelMode::Ship
        }
        TravelMode::Bike => *edge_mode != TravelMode::Foot,
        TravelMode::Foot => true,
        _ => panic!(),
    }
}
