use std::rc::Rc;

use serde::Serializer;
use serde::ser::Serialize;
use serde::ser::SerializeStruct;

use super::Graph;
use super::CoarsenLink;
pub struct GraphPos {
  g: Rc<Graph>,
  pos: Vec<f32>,
  dim: u8,
}

impl GraphPos {
  pub fn new(g: Rc<Graph>, pos: Vec<f32>, dim: u8) -> Result<Self, &'static str> {
    
    if g.len() * (dim as usize) != pos.len() {
      return Err("The position array length doesn't match with |V| of the graph.");
    }

    Ok(GraphPos { g, pos, dim})
  }

  pub fn coarsen(&self) -> Result<(Rc<Graph>, CoarsenLink, GraphPos), &'static str> {
    let (g1, link) = self.g.coarsen();
    let new_len = g1.len();
    let mapping = link.mapping();
    let new_dim: usize = self.dim as usize;

    let pos: &Vec<f32> = &self.pos;
    let mut new_pos: Vec<f32> = Vec::with_capacity(new_len * new_dim);
    
    for i0 in mapping.iter() {
      for d in 0..new_dim {
        new_pos.push(pos[i0 * new_dim + d]);
      }
    }
    let g1_pos = GraphPos::new(Rc::clone(&g1), new_pos, new_dim as u8)?;

    Ok((g1, link, g1_pos))
  }
}

impl Serialize for GraphPos {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Graph", 3)?;
        let edges = self.g.to_edge_list();
        
        state.serialize_field("indices", &edges)?;
        state.serialize_field("position", &self.pos)?;
        state.serialize_field("Dim", &self.dim)?;
        state.end()
    }
}