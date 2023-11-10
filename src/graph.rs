use std::rc::{Rc, Weak};

mod build;
mod util;
mod pos;

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

pub struct Graph {
  adj: Vec<Vec<usize>>,
  me: Weak<Self>
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

  pub fn coarsen(&self) -> CoarsenLink {
    let len: usize = self.len();
    
    let mut keep: Vec<usize> = vec![0; len];
    let mut len1: usize = 0;
    for idx in 0..len {
      if keep[idx] == len {
        continue;
      }
      
      len1 += 1;
      let nexts: &Vec<usize> = self.get_nexts(idx);
      for next in nexts.iter() {
        keep[*next] = len;
      }
    }

    let mut visited: Vec<bool> = vec![false; len];
    let mut mapping: Vec<usize> = Vec::with_capacity(len1);
    let mut idx: usize = 0;
    let mut adj_dis2: Vec<Vec<usize>> = Vec::with_capacity(len1);
    let mut adj_dis3: Vec<Vec<usize>> = Vec::with_capacity(len1);
    for idx1 in 0..len1 {
      while idx < len && keep[idx] == len {
        idx += 1;
      }
      mapping.push(idx);
      keep[idx] = idx1;

      visited[idx] = true;
      let ns1: &Vec<usize> = self.get_nexts(idx);
      util::set_vec_at_idxs_to(&mut visited, ns1, true);

      let ns2: Vec<usize> = util::get_nexts_nexts(self, &mut visited, &ns1);
      let ns3: Vec<usize> = util::get_nexts_nexts(self, &mut visited, &ns2);

      util::set_vec_at_idxs_to(&mut visited, &ns3, false);
      util::set_vec_at_idxs_to(&mut visited, &ns2, false);
      util::set_vec_at_idxs_to(&mut visited, ns1, false);

      visited[idx] = false;
      adj_dis2.push(ns2);
      adj_dis3.push(ns3);
      idx += 1;
    }
    
    let mut adj1: Vec<Vec<usize>> = Vec::with_capacity(len1);
    for idx1 in 0..len1 {
      let mut nexts1: Vec<usize> = Vec::new();
      for next in adj_dis2[idx1].iter() {
        if keep[*next] < len {
          // print!("{} ", keep[*next]);
          nexts1.push(keep[*next]);
        }
      }
      adj1.push(nexts1); 
    }

    let (count, belong) = util::get_groups(&adj1);
    if count > 0 {
      for i1 in 0..len1 {
        let ref mut nexts1 = adj1[i1];
        
        for j0 in adj_dis3[i1].iter() {
          if keep[*j0] < len {
            let j1 = keep[*j0];
            // print!("{} ", keep[*next]);
            if belong[j1] != belong[i1] {
              nexts1.push(j1);
            }
          }
        }
      }
    }
    CoarsenLink { g0: Rc::clone(&self.me.upgrade().unwrap()), g1:Graph::from_adj(adj1), mapping}
  }
}
