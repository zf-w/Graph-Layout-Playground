pub fn sequential_picker(adj: &Vec<Vec<usize>>, save: &Vec<bool>) -> (usize, Vec<usize>) {
  let len: usize = adj.len();
  let mut keep: Vec<usize> = vec![0; len];
  let mut len1: usize = 0;
  for idx in 0..len {
    if keep[idx] == len {
      continue;
    }
    
    len1 += 1;
    if !save[idx] {
      let nexts: &Vec<usize> = &adj[idx];
      for next in nexts.iter() {
        keep[*next] = len;
      }
    }
  }
  (len1, keep)
}

pub fn degree_sort_picker(adj: &Vec<Vec<usize>>, save: &Vec<bool>) -> (usize, Vec<usize>) {
  let len: usize = adj.len();
  let mut keep: Vec<usize> = vec![0; len];
  let mut len1: usize = 0;
  let mut degrees: Vec<(usize, usize)> = Vec::with_capacity(len);
  for idx in 0..len {
    if !save[idx] {
      degrees.push((adj[idx].len(), idx));
    } else {
      len1 += 1;
    }
  }

  degrees.sort();

  for (_, idx) in degrees.iter() {
    if keep[*idx] == len {
      continue;
    }
    
    len1 += 1;
    if !save[*idx] {
      let nexts: &Vec<usize> = &adj[*idx];
      for next in nexts.iter() {
        keep[*next] = len;
      }
    }
  }
  (len1, keep)
}