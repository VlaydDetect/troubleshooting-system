mod block;
mod subgraph;

use std::fmt::{Display, Formatter, Write};
use ndarray::Array2;
use crate::error::{Result, Error};
use crate::prelude::f;

pub struct Graph<Id: Display + PartialEq> {
    adjacency_list: Array2<bool>,
    nodes: Vec<Id>,
    n: usize,
}

impl<Id: Display + PartialEq> Graph<Id> {
    pub fn new(n: usize, data: &[&[bool]]) -> Self {
        let data = data.iter().map(|&elem| elem.to_vec()).flatten().collect::<Vec<_>>();
        Self {
            adjacency_list: Array2::from_shape_vec((n, n), data).unwrap(),
            nodes: Vec::with_capacity(n),
            n,
        }
    }

    pub fn falses(n: usize) -> Self {
        let data = (0..n * n).map(|_| false).collect();
        Self {
            adjacency_list: Array2::from_shape_vec((n, n), data).unwrap(),
            nodes: Vec::with_capacity(n),
            n,
        }
    }

    pub fn add_node(&mut self, node: Id) -> Result<()> {
        if self.nodes.contains(&node) {
            return Err(Error::GraphError(f!("Node with id {} already exists", node)));
        }

        self.nodes.push(node);
        Ok(())
    }

    pub fn add_edge(&mut self, a: Id, b: Id) -> Result<()> {
        if !self.nodes.contains(&a) || !self.nodes.contains(&b) {
            return Err(Error::GraphError("Graph doesn't contain nodes of edge".to_string()));
        }

        let a_id = self.nodes.
            iter()
            .enumerate()
            .find_map(|node| { return if a.eq(&node.1) { Some(node.0) } else { None }; })
            .ok_or(Error::GraphError(f!("Cannot find node with id {}", a)))?;
        let b_id = self.nodes.
            iter()
            .enumerate()
            .find_map(|node| { return if b.eq(&node.1) { Some(node.0) } else { None }; })
            .ok_or(Error::GraphError(f!("Cannot find node with id {}", b)))?;

        self.adjacency_list[[a_id, b_id]] = true;

        Ok(())
    }
}

impl<Id: Display + PartialEq + std::fmt::Debug> Display for Graph<Id> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("   ")?;

        f.write_fmt(format_args!("{:?}\n", self.nodes))?;
        for i in 0..self.adjacency_list.nrows() {
            f.write_fmt(format_args!("{:?} ", self.nodes[i]))?;
            f.write_fmt(format_args!("{}\n", self.adjacency_list.row(i)))?;
        }

        Ok(())
    }
}