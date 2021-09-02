use std::collections::HashMap;

use itertools::Itertools;
use ordered_float::OrderedFloat;

use crate::{
    algorithms::{
        convex_hull, dijkstra, find_path,
        graph::{Edge, EdgeRef, Graph, Node, NodeIndex},
    },
    models::{Command, Link, Place, TravelMode},
};

#[derive(Clone)]
pub struct CommandProcessor {
    graph: Graph<Place, Link>,
    id_map: HashMap<i32, NodeIndex>,
}

impl CommandProcessor {
    pub fn new(graph: Graph<Place, Link>, id_map: HashMap<i32, NodeIndex>) -> Self {
        Self { graph, id_map }
    }

    pub fn process(&self, command: Command) -> String {
        match command {
            Command::MaxDist => self.max_dist(),
            Command::MaxLink => self.max_link(),
            Command::FindDist(a, b) => self.find_distance(a, b),
            Command::FindNeighbour(place) => self.find_neighbour(place),
            Command::Check(mode, nodes) => self.check(mode, &nodes),
            Command::FindShortestRoute(mode, start, destination) => {
                self.find_shortest_route(mode, start, destination)
            }
            Command::FindRoute(mode, start, dest) => self.find_route(mode, start, dest),
        }
    }

    fn max_dist(&self) -> String {
        let places: Vec<Place> = self
            .graph
            .raw_nodes()
            .into_iter()
            .map(|place| place.data.clone())
            .collect();

        let hull = &convex_hull(places.as_slice());

        let mut pair = (&hull[0], &hull[1]);
        let mut max_dist = -1.0;

        for (a, b) in hull.into_iter().tuple_combinations() {
            let dist = self.distance(a, b);
            if dist > max_dist {
                max_dist = dist;
                pair = (a, b);
            }
        }

        let dist = max_dist.sqrt() / 1000.0;
        format!("MaxDist\n{}, {}, {}", pair.0, pair.1, dist)
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

        format!("MaxLink\n{},{},{:.1}", a, b, dist)
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
            let next_node = self.graph.get_node(next_id).unwrap();

            let outcome = if connects { "PASS" } else { "FAIL" };
            output = format!(
                "{}\n{},{},{}",
                output, current_node.data.id, next_node.data.id, outcome
            );
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

    fn find_distance(&self, a: i32, b: i32) -> String {
        let node_a = self.index_to_node(a);
        let node_b = self.index_to_node(b);

        let dist = self.distance(&node_a.data, &node_b.data);
        format!(
            "FindDist\n{},{},{:.3}",
            node_a.data.name, node_b.data.name, dist
        )
    }

    fn distance(&self, a: &Place, b: &Place) -> f64 {
        let dx = a.eastings - b.eastings;
        let dy = a.northings - b.northings;

        let dist = (dx * dx + dy * dy).sqrt();

        dist / 1000.0
    }

    fn index_to_node(&self, id: i32) -> &Node<Place> {
        let index = self.id_map.get(&id).unwrap();
        self.graph.get_node(*index).unwrap()
    }

    fn edge_to_distance(&self, edge: &Edge<Link>) -> f64 {
        let a = self.graph.get_node(edge.source).unwrap();
        let b = self.graph.get_node(edge.destination).unwrap();

        self.distance(&a.data, &b.data)
    }

    fn find_route(&self, mode: TravelMode, start: i32, goal: i32) -> String {
        let start_node = self.id_map.get(&start).unwrap();
        let goal_node = self.id_map.get(&goal).unwrap();

        let routes = find_path(
            &self.graph,
            *start_node,
            *goal_node,
            |a, b| {
                let dx = a.eastings - b.eastings;
                let dy = a.northings - b.northings;

                dx * dx + dy * dy
            },
            |x| can_traverse(&mode, &x.mode),
        );

        let mut nodes = vec![goal_node];
        let mut curr = goal_node;

        let mut output = format!("FindRoute {} {} {}", mode, start, goal);

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
