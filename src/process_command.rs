use std::collections::HashMap;

use geoutils::Location;
use ordered_float::OrderedFloat;

use crate::{
    dijkstra::dijkstra,
    graph::{Edge, EdgeRef, Graph, NodeIndex},
    models::{Command, Link, PlaceCopy, TravelMode},
};

pub struct CommandProcessor {
    graph: Graph<PlaceCopy, Link>,
    id_map: HashMap<i32, NodeIndex>,
}

impl CommandProcessor {
    pub fn new(graph: Graph<PlaceCopy, Link>, id_map: HashMap<i32, NodeIndex>) -> Self {
        Self { graph, id_map }
    }

    pub fn process(&self, command: Command) -> String {
        match command {
            Command::FindNeighbour(place) => self.find_neighbour(place),
            Command::FindShortestRoute(mode, start, destination) => {
                self.find_shortest_route(mode, start, destination)
            }
            Command::Check(mode, nodes) => self.check(mode, &nodes),
            Command::MaxLink => self.max_link(),
            _ => "Not impl".to_string(),
            Command::MaxDist => "max dist".to_string(),
            Command::FindDist(_, _) => todo!(),
            Command::FindRoute(_, _, _) => todo!(),
        }
    }

    fn find_neighbour(&self, id: i32) -> String {
        let node_id = self.id_map.get(&id).unwrap();
        let node = self.graph.get_node(*node_id).unwrap();

        let mut output = format!("FindNeighbour {}", node.data.id);

        for neighbour in self.graph.edges(*node_id) {
            let dest = self.graph.get_node(neighbour.nodes[1]).unwrap();
            output = format!("{}\n{}", output, dest.data.id);
        }

        output
    }

    fn find_shortest_route(&self, mode: TravelMode, start: i32, goal: i32) -> String {
        let start_node = self.id_map.get(&start).unwrap();
        let goal_node = self.id_map.get(&goal).unwrap();

        let routes = dijkstra(
            &self.graph,
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

        for i in nodes {
            let node = self.graph.get_node(*i).unwrap();
            output = format!("{}\n{}", output, &node.data.to_string());
        }

        output
    }

    fn check(&self, mode: TravelMode, node_ids: &Vec<i32>) -> String {
        let indexes: Vec<&usize> = node_ids
            .into_iter()
            .map(|x| self.id_map.get(&x).unwrap())
            .collect();

        let string_ids: Vec<String> = node_ids.into_iter().map(|x| x.to_string()).collect();
        let mut output = format!("Check {} {}", mode, string_ids.join(" "));

        for i in 0..indexes.len() - 1 {
            let current_id = *indexes[i];
            let next_id = *indexes[i + 1];

            let connects = self
                .graph
                .edges(current_id)
                .filter(|x| can_traverse(&mode, &x.data.mode))
                .any(|x| x.destination() == next_id);

            let current_node = self.graph.get_node(current_id).unwrap();
            let next_node = *self.graph.get_node(next_id).unwrap();

            let outcome = if connects { "PASS" } else { "FAIL" };
            output = format!(
                "{}\n{},{},{}",
                output, current_node.data.id, next_node.data.id, outcome
            );
        }

        output
    }

    fn max_link(&self) -> String {
        let max = self
            .graph
            .raw_edges()
            .iter()
            .max_by_key(|edge| {
                let dist = self.edge_to_distance(edge);
                OrderedFloat(dist)
            })
            .unwrap();

        let dist = self.edge_to_distance(max);
        let a = self.graph.get_node(max.source).unwrap().data.id;
        let b = self.graph.get_node(max.destination).unwrap().data.id;

        format!("{},{},{}", a, b, dist)
    }

    fn edge_to_distance(&self, edge: &Edge<Link>) -> f64 {
        let a = self.graph.get_node(edge.source).unwrap();
        let b = self.graph.get_node(edge.destination).unwrap();

        let c = Location::new(a.data.latitude, a.data.longitude);
        let d = Location::new(b.data.latitude, b.data.longitude);

        c.distance_to(&d).unwrap().meters()
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
    }
}
