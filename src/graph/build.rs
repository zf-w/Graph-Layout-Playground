use std::collections::BTreeMap;
use std::collections::BTreeSet;

/// ## Edges Cleaning
/// 
/// ### Background
/// 
/// There might be duplicated edges and self-edges from the user input.
/// 
/// ### Design
/// 
/// Firstly, this function will make sure all the edges are having 
/// smaller indices at index zero and bigger indices at index one.
/// 
/// Then, it loops through the edges and inserts them into a BTreeSet to remove all the duplicates.
/// 
/// Finally, it put the cleaned edges back into the original vector, truncate its size, 
/// and return its ownership back.
fn clean_edges(mut edges: Vec<usize>) -> Vec<usize> {
  let mut edge_set: BTreeSet<(usize, usize)> = BTreeSet::new();
  for i in 0..(edges.len() / 2) {
    let i2: usize = i * 2;
    let a: usize = edges[i2];
    let b: usize = edges[i2 + 1];
    if a != b {
      edge_set.insert(if a < b {
        (a, b)
      } else {
        (b, a)
      });
    }
  }
  for (i, (a, b)) in edge_set.iter().enumerate() {
    let i2: usize = i * 2;
    edges[i2] = *a;
    edges[i2 + 1] = *b;
  }
  edges.truncate(edge_set.len() * 2);
  edges
}

pub fn edges_to_adj(edges: Vec<usize>) -> Vec<Vec<usize>> {
  let edges = clean_edges(edges);
  let mut mp : BTreeMap<usize, usize> = BTreeMap::new();
  for i in edges.iter() {
    match mp.get_mut(i) {
      Some(v) => {
        *v += 1;
      },
      None => {
        mp.insert(*i, 1);
      }
    }
  }
  let len: usize = mp.len();
  let mut adj: Vec<Vec<usize>> = Vec::with_capacity(len);
  
  for (i, (_k, v)) in mp.iter_mut().enumerate() {
    adj.push(Vec::with_capacity(*v));
    *v = i;
  }

  for i in 0..(edges.len() / 2) {
    let i2: usize = i * 2;
    let a: usize = *(mp.get(&edges[i2]).unwrap());
    let b: usize = *(mp.get(&edges[i2 + 1]).unwrap());
    if a != b {
      adj[a].push(b);
      adj[b].push(a);
    }
  }
  adj
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_clean_edges() {
    let dirty_edges = vec![800, 800, 3, 4, 4, 3];
    assert_eq!(vec![3, 4], clean_edges(dirty_edges));
  }

  #[test]
  fn test_edges_to_adj() {
    let edges0: Vec<usize> = vec![0, 1, 2, 3];
    let edges1: Vec<usize> = vec![1, 2, 3, 800];
    let true_adj: Vec<Vec<usize>> = vec![vec![1], vec![0], vec![3], vec![2]];
    assert_eq!(true_adj, edges_to_adj(edges0));
    assert_eq!(true_adj, edges_to_adj(edges1));
  }

  #[test]
  fn test_edges_to_adj_capacity_check() {
    let edges: Vec<usize> = vec![1, 2, 3, 800];
    
    fn capacity_check(v: &Vec<Vec<usize>>) {
      assert_eq!(v.capacity(), v.len());
      for i in v.iter() {
        assert_eq!(i.capacity(), i.len());
      }
    }

    let adj: Vec<Vec<usize>> = edges_to_adj(edges);

    capacity_check(&adj);
  }
}