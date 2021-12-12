use std::cmp::max;

#[derive(Clone, Copy, Debug)]
pub struct Node<N> {
    pub data: N,
    outgoing: EdgeIndex,
    incoming: EdgeIndex,
}

//TODO convert to two arrays and add functions
#[derive(Clone, Copy, Debug)]
pub struct Edge<E> {
    pub data: E,

    pub source: NodeIndex,
    pub destination: NodeIndex,

    outgoing: EdgeIndex,
    incoming: EdgeIndex,
}

#[derive(Debug, Clone)]
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

    pub fn get_node(&self, index: NodeIndex) -> Option<&Node<N>> {
        self.nodes.get(index)
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
        data: E,
    ) -> EdgeIndex {
        let (source_node, destination_node) =
            Graph::<N, E>::get_nodes(&mut self.nodes, source_index, dest_index);

        let edge = Edge {
            data,
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

    pub fn raw_nodes(&self) -> &[Node<N>] {
        &self.nodes
    }

    pub fn raw_edges(&self) -> &[Edge<E>] {
        &self.edges
    }

    pub fn is_adjacent(&self, a: NodeIndex, b: NodeIndex) -> bool {
        self.edges(a).any(|edge| edge.destination() == b)
    }

    fn get_nodes(
        nodes: &mut Vec<Node<N>>,
        index_a: NodeIndex,
        index_b: NodeIndex,
    ) -> (&mut Node<N>, &mut Node<N>) {
        assert!(
            !(index_a == index_b),
            "Graph does not support self referencing"
        );

        assert!(
            !(max(index_a, index_b) > nodes.len()),
            "Index out of bounds"
        );

        unsafe {
            let ptr = nodes.as_mut_ptr();
            let node_a = &mut *ptr.add(index_a);
            let node_b = &mut *ptr.add(index_b);

            (node_a, node_b)
        }
    }
}

impl<N, E> Default for Graph<N, E> {
    fn default() -> Self {
        Self::new()
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
    pub index: EdgeIndex,
    pub nodes: [NodeIndex; 2],
    pub data: &'a E,
}

pub trait EdgeRef {
    fn source(&self) -> NodeIndex;
    fn destination(&self) -> NodeIndex;
}
//     fn index(&self) -> EdgeIndex;
//     fn data(&self) -> &'a E;
// }

impl<'a, E> EdgeRef for EdgeReference<'a, E> {
    fn source(&self) -> NodeIndex {
        self.nodes[0]
    }
    fn destination(&self) -> NodeIndex {
        self.nodes[1]
    }
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
                    data: &edge.data,
                });
            }
        };

        if let EdgeIndex::Index(index) = self.incoming {
            let val = self.edges.get(index);

            if let Some(edge) = val {
                self.incoming = edge.incoming;

                return Some(EdgeReference {
                    index: EdgeIndex::Index(index),
                    // Swap nodes
                    nodes: [edge.destination, edge.source],
                    data: &edge.data,
                });
            }
        };

        None
    }
}
