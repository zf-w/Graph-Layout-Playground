use std::rc::Rc;
use crate::Graph;

mod build;
mod util;
mod pos;
mod coarsen;

pub use pos::GraphPos;

pub struct CoarsenLink {
  g0: Rc<Graph>,
  g1: Rc<Graph>,
  mapping: Vec<usize>,
}

impl CoarsenLink {
  pub fn mapping(&self) -> &Vec<usize> {
    &self.mapping
  }
}

impl Graph {
  pub fn from_edge_list(edges: Vec<usize>) -> Rc<Self> {
    Rc::new_cyclic(|me| {
      Graph { adj: build::edges_to_adj(edges),me: me.clone() }
    })
  }

  pub fn to_edge_list(&self) -> Vec<usize> {
    let len: usize = self.adj.iter().map(|v| {v.len()}).sum();
    let mut edges: Vec<usize> = Vec::with_capacity(len);
    for (i, ls) in self.adj.iter().enumerate() {
      for j in ls.iter() {
        if i < *j {
          edges.push(i);
          edges.push(*j);
        }
      }
    }
    edges
  }

  pub fn from_adj(adj: Vec<Vec<usize>>) -> Rc<Self> {
    Rc::new_cyclic(|me| {
      Graph { adj, me: me.clone() }
    })
  }

  pub fn len(&self) -> usize {
    self.adj.len()
  }

  pub fn get_nexts(&self, idx: usize) -> &Vec<usize> {
    if idx >= self.len() {
      panic!("Idx out of bounds...");
    }
    &self.adj[idx]
  }
}
