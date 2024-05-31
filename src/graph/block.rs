use std::collections::HashMap;
use crate::graph::subgraph::Subgraph;
use super::Graph;
use crate::error::Result;

pub struct BlocksGraph {
    graph: Graph<String>,
    subgraphs: HashMap<String, Subgraph>,
}

impl BlocksGraph {
    pub fn new(n: usize) -> Self {
        Self {
            graph: Graph::falses(n),
            subgraphs: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: String) -> Result<()> {
        self.graph.add_node(node)
    }

    pub fn add_edge(&mut self, a: String, b: String) -> Result<()> {
        self.graph.add_edge(a, b)
    }

    pub fn add_subgraph(&mut self, block: String, subgraph: Subgraph) -> Option<Subgraph> {
        self.subgraphs.insert(block, subgraph)
    }
}