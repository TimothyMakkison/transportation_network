use std::cmp::max;

#[derive(Clone, Copy, Debug)]
struct Node<N> {
    data: N,
    outgoing: EdgeIndex,
    incoming: EdgeIndex,
}

#[derive(Clone, Copy, Debug)]
struct Edge<E> {
    weight: E,

    source: NodeIndex,
    destination: NodeIndex,

    outgoing: EdgeIndex,
    incoming: EdgeIndex,
}

#[derive(Debug)]
pub struct Graph<N, E> {
    nodes: Vec<Node<N>>,
    edges: Vec<Edge<E>>,
}

impl<N, E> Graph<N, E> {
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, data: N) -> NodeIndex {
        let node = Node {
            data,
            outgoing: EdgeIndex::None,
            incoming: EdgeIndex::None,
        };
        let index = self.nodes.len();
        self.nodes.push(node);
        index
    }

    pub fn add_edge(
        &mut self,
        source_index: NodeIndex,
        dest_index: NodeIndex,
        weight: E,
    ) -> EdgeIndex {
        let (source_node, destination_node) =
            Graph::<N, E>::get_nodes(&mut self.nodes, source_index, dest_index);

        let edge = Edge {
            weight,
            source: source_index,
            destination: dest_index,
            outgoing: source_node.outgoing,
            incoming: destination_node.incoming,
        };

        let index = EdgeIndex::Index(self.edges.len());

        source_node.outgoing = index;
        destination_node.incoming = index;

        self.edges.push(edge);
        index
    }

    pub fn edges(&self, node_index: NodeIndex) -> Edges<E> {
        let node = self.nodes.get(node_index);
        Edges {
            edges: &self.edges,
            outgoing: match node {
                Some(node) => node.outgoing,
                None => EdgeIndex::None,
            },
            incoming: match node {
                Some(node) => node.incoming,
                None => EdgeIndex::None,
            },
        }
    }

    fn get_nodes(
        nodes: &mut Vec<Node<N>>,
        index_a: NodeIndex,
        index_b: NodeIndex,
    ) -> (&mut Node<N>, &mut Node<N>) {
        if index_a == index_b {
            panic!("Graph does not support self referencing");
        }

        if max(index_a, index_b) > nodes.len() {
            panic!("Index out of bounds");
        }

        unsafe {
            let ptr = nodes.as_mut_ptr();
            let node_a = &mut *ptr.add(index_a);
            let node_b = &mut *ptr.add(index_b);

            (node_a, node_b)
        }
    }
}

pub type NodeIndex = usize;

#[derive(Debug, Clone, Copy)]
pub enum EdgeIndex {
    Index(usize),
    None,
}
impl EdgeIndex {
    pub fn unwrap(self) -> usize {
        match self {
            EdgeIndex::Index(x) => x,
            EdgeIndex::None => panic!("Edge index is not defined."),
        }
    }
}

pub enum EdgeType {
    Outgoing,
    Incoming,
}

#[derive(Debug)]
pub struct EdgeReference<'a, E> {
    index: EdgeIndex,
    nodes: [NodeIndex; 2],
    weight: &'a E,
}

pub struct Edges<'a, E> {
    edges: &'a [Edge<E>],
    outgoing: EdgeIndex,
    incoming: EdgeIndex,
}

// Iterator for edges. Should take the next edgeindex.
// Should check if none -> return none
// Else try and get value from edges

impl<'a, E> Iterator for Edges<'a, E> {
    type Item = EdgeReference<'a, E>;

    fn next(&mut self) -> Option<Self::Item> {
        if let EdgeIndex::Index(index) = self.outgoing {
            let val = self.edges.get(index);

            if let Some(edge) = val {
                self.outgoing = edge.outgoing;

                return Some(EdgeReference {
                    index: EdgeIndex::Index(index),
                    nodes: [edge.source, edge.destination],
                    weight: &edge.weight,
                });
            }
        };

        if let EdgeIndex::Index(index) = self.incoming {
            let val = self.edges.get(index);

            if let Some(edge) = val {
                self.incoming = edge.incoming;

                return Some(EdgeReference {
                    index: EdgeIndex::Index(index),
                    nodes: [edge.destination, edge.source],
                    weight: &edge.weight,
                });
            }
        };

        return None;
    }
}
