use std::rc::Rc;

mod picker;

use super::{Graph, CoarsenLink};
use super::util;

impl Graph {
  pub fn coarsen(&self) -> CoarsenLink {
    let len: usize = self.len();
    let lb: usize = 10;

    let (sizes0, belong0) = util::get_groups(&self.adj);
    let mut save: Vec<bool> = vec![false; len];
    for (i, v) in save.iter_mut().enumerate() {
      if sizes0[belong0[i]] < lb {
        *v = true;
      }
    }

    let (len1, mut keep) = picker::degree_sort_picker(&self.adj, &save);

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

      if !save[idx] {
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
      } else {
        adj_dis2.push(Vec::new());
        adj_dis3.push(Vec::new());
      }
      idx += 1;
    }
    
    let mut adj1: Vec<Vec<usize>> = Vec::with_capacity(len1);
    for idx1 in 0..len1 {
      let idx = mapping[idx1]; // Bug found here, mapping in the wrong direction
      let mut nexts1: Vec<usize> = Vec::new();
      if !save[idx] {
        for next in adj_dis2[idx1].iter() {
          if keep[*next] < len {
            // print!("{} ", keep[*next]);
            nexts1.push(keep[*next]);
          }
        }
      } else {
        for next in self.get_nexts(idx).iter() {
          if keep[*next] < len {
            // print!("{} ", keep[*next]);
            nexts1.push(keep[*next]);
          }
        }
      }
      
      adj1.push(nexts1); 
    }

    let (_, belong) = util::get_groups(&adj1);
    // if sizes.len() > 0 {
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
      // }
    }
    CoarsenLink { g0: Rc::clone(&self.me.upgrade().unwrap()), g1:Graph::from_adj(adj1), mapping}
  }
}