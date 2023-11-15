pub fn normalize2d_center(pos: &Vec<f32>, dim: usize) -> Vec<f32> {
  let size: usize = pos.len() / dim;
  let d2: usize = 2;
  let mut res: Vec<f32> = Vec::with_capacity(size * d2);
  for i in 0..size {
    res.push(pos[i * dim + 0]);
    res.push(pos[i * dim + 1]);
  }

  let mut x: f32 = 0.0;
  let mut y: f32 = 0.0;

  for i in 0..size {
    x += res[i * d2];
    y += res[i * d2 + 1];
  }

  x /= size as f32;
  y /= size as f32;

  for i in 0..size {
    res[i * d2] -= x;
    res[i * d2 + 1] -= y;
  }
  let mut max_r: f32 = 0.0;
  for i in 0..size {
    let curr = res[i * d2].powi(2) + res[i * d2 + 1].powi(2);
    max_r = max_r.max(curr);
  }

  max_r = max_r.sqrt() * 1.1;

  for v in res.iter_mut() {
    *v /= max_r;
    *v /= 2.0;
    *v += 0.5;
  }

  res
}

pub fn normalize2d_full(pos: &Vec<f32>, dim: usize) -> Vec<f32> {
  let size: usize = pos.len() / dim;
  let d2: usize = 2;
  let mut res: Vec<f32> = Vec::with_capacity(size * d2);
  for i in 0..size {
    res.push(pos[i * dim + 0]);
    res.push(pos[i * dim + 1]);
  }

  let mut x_min: f32 = res[0];
  let mut x_max: f32 = res[0];
  let mut y_min: f32 = res[1];
  let mut y_max: f32 = res[1];

  for i in 0..size {
    x_min = x_min.min(res[i * d2]);
    x_max = x_max.max(res[i * d2]);
    y_min = y_min.min(res[i * d2 + 1]);
    y_max = y_max.max(res[i * d2 + 1]);
  }

  let x_range = x_max - x_min;
  let y_range = y_max - y_min;
  let mut range = x_range.max(y_range);
  if range == 0.0 {
    range = 1.0;
  }

  for i in 0..size {
    res[i * d2] = 0.5 + (res[i * d2] - x_min - (x_range / 2.0)) / range;
    res[i * d2 + 1] = 0.5 + (res[i * d2 + 1] - y_min - (y_range / 2.0)) / range;
  }

  res
}