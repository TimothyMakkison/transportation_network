pub mod graph {
    use std::cmp::max;

    #[derive(Clone, Debug)]
    struct Node<N> {
        data: N,
        outgoing: EdgeIndex,
        incoming: EdgeIndex,
    }

    #[derive(Clone, Debug)]
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
            let index = NodeIndex::Index(self.nodes.len());
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

        fn get_nodes(
            nodes: &mut Vec<Node<N>>,
            a: NodeIndex,
            b: NodeIndex,
        ) -> (&mut Node<N>, &mut Node<N>) {
            let index_a = a.unwrap();
            let index_b = b.unwrap();

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

    #[derive(Debug, Clone, Copy)]
    pub enum NodeIndex {
        Index(usize),
        None,
    }

    impl NodeIndex {
        pub fn unwrap(self) -> usize {
            match self {
                NodeIndex::Index(x) => x,
                NodeIndex::None => panic!("Node index is not defined."),
            }
        }
    }

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
}
