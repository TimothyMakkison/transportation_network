use std::{collections::HashMap, fmt::Display};

use crate::{
    dijkstra::dijkstra,
    graph::{Graph, NodeIndex},
    models::{Command, Link, PlaceCopy, TravelMode},
};

pub fn process(
    graph: &Graph<PlaceCopy, Link>,
    node_map: &HashMap<i32, NodeIndex>,
    command: Command,
) -> String {
    match command {
        Command::MaxDist => "max dist".to_string(),
        Command::FindNeighbour(place) => place.to_string(),
        Command::FindShortestRoute(_, _, _) => find_shortest_route(&graph, node_map, command),
        _ => "Not impl".to_string(),
        Command::MaxLink => todo!(),
        Command::FindDist(_, _) => todo!(),
        Command::Check(_, _) => todo!(),
        Command::FindRoute(_, _, _) => todo!(),
    }
}

fn find_shortest_route(
    graph: &Graph<PlaceCopy, Link>,
    id_map: &HashMap<i32, NodeIndex>,
    command: Command,
) -> String {
    if let Command::FindShortestRoute(mode, start, goal) = command {
        let start_node = id_map.get(&start).unwrap();
        let goal_node = id_map.get(&goal).unwrap();

        let routes = dijkstra(
            &graph,
            *start_node,
            Some(*goal_node),
            |_| 1,
            |x| can_traverse(&mode, &x.mode),
        );

        let mut nodes = vec![goal_node];
        let mut curr = goal_node;

        let mut output = format!("FindShortestRoute {} {} {}", mode, start, goal);

        while curr != start_node {
            match routes.get(curr) {
                Some(pair) => {
                    curr = &pair.1;
                    nodes.push(curr);
                }
                None => {
                    output = format!("{} \nFail", output);

                    return output;
                }
            }
        }
        nodes.reverse();

        println!("{:?}", nodes);
        for i in nodes {
            let node = graph.get_node(*i).unwrap();
            output = format!("{}\n{}", output, &node.data.to_string());
        }

        output
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
