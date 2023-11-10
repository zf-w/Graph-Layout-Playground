use std::collections::VecDeque;

pub fn set_vec_at_idxs_to<T: Clone>(v: &mut Vec<T>, idxs: &Vec<usize>, val: T) {
  for idx in idxs.iter() {
    v[*idx] = val.clone();
  }
}

use super::Graph;

pub fn get_nexts_nexts(g: &Graph, v: &mut Vec<bool>, ns: &Vec<usize>) -> Vec<usize> {
  let mut ans: Vec<usize> = Vec::new();
  for n in ns.iter() {
    for nn in g.get_nexts(*n).iter() {
      if v[*nn] == false {
        v[*nn] = true;
        ans.push(*nn);
      }
    }
  }
  ans
}

pub fn get_groups(adj: &Vec<Vec<usize>>) ->(usize, Vec<usize>) {
  let len: usize = adj.len();
  let mut v: Vec<bool> = vec![false; len];
  let mut res: Vec<usize> = Vec::with_capacity(len);
  for i in 0..len {
    res.push(i);
  }
  fn iter_bfs_nexts(adj: &Vec<Vec<usize>>, q: &mut VecDeque<usize>, v: &mut Vec<bool>, res: &mut Vec<usize>, start: usize) {
    let len: usize = q.len();
    for _ in 0..len {
      let curr = q.front().unwrap().clone();
      q.pop_front();
      let ref nexts = adj[curr];
      for next in nexts.iter() {
        if v[*next] == false {
          res[*next] = start;
          v[*next] = true;
          q.push_back(*next);
        }
      }
    }
  }
  let mut count = 0;
  for i in 0..len {
    if res[i] != i {
      continue;
    }
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(i);
    v[i] = true;
    while !q.is_empty() {
      iter_bfs_nexts(&adj, &mut q, &mut v, &mut res, count);
    }
    count += 1;
  }

  (count, res)
}