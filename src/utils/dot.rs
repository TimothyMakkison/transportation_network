use std::fmt;

use crate::algorithms::graph::Graph;

pub struct Dot<'a, N, E> {
    graph: Graph<N, E>,
    node_func: &'a dyn Fn(&mut fmt::Formatter, &N) -> fmt::Result,
    edge_func: &'a dyn Fn(&mut fmt::Formatter, &E) -> fmt::Result,
    subgraphs: &'a [&'a dyn Fn(&mut fmt::Formatter) -> fmt::Result],
}

impl<'a, N, E> Dot<'a, N, E> {
    pub fn new(
        graph: Graph<N, E>,
        node_func: &'a dyn Fn(&mut fmt::Formatter, &N) -> fmt::Result,
        edge_func: &'a dyn Fn(&mut fmt::Formatter, &E) -> fmt::Result,
        subgraphs: &'a [&'a dyn Fn(&mut fmt::Formatter) -> fmt::Result],
    ) -> Self {
        Self {
            graph,
            node_func,
            edge_func,
            subgraphs,
        }
    }
}

impl<'a, N, E> fmt::Display for Dot<'a, N, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "graph {{")?;
        for (index, node) in self.graph.raw_nodes().into_iter().enumerate() {
            write!(f, "{} [", index)?;
            // node.data.fmt(f)?;
            // write!(f, "]\n")?;
            (self.node_func)(f, &node.data)?;
            write!(f, "]\n")?;
        }

        for edge in self.graph.raw_edges() {
            write!(f, "{} -- {} [", edge.source, edge.destination)?;
            // edge.data.fmt(f)?;
            (self.edge_func)(f, &edge.data)?;
            write!(f, "]\n")?;
        }

        for sub_graph in self.subgraphs {
            (sub_graph)(f)?;
        }

        writeln!(f, "}}")?;

        Ok(())
    }
}
