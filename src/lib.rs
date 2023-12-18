pub mod graph;
pub mod img;

use std::rc::Weak;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Input {
  pub indices: Vec<usize>,
  pub position: Option<Vec<f32>>,
  pub dim: Option<u8>
}

/// ## Graph
pub struct Graph {
  adj: Vec<Vec<usize>>,
  me: Weak<Self>
}