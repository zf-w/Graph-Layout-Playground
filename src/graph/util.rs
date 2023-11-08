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