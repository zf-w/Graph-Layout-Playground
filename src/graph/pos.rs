use std::rc::Rc;

use serde::Serializer;
use serde::ser::Serialize;
use serde::ser::SerializeStruct;

use crate::img::Img;

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
    let link = self.g.coarsen();
    let g1 = Rc::clone(&link.g1);
    let new_len = g1.len();
    let mapping = link.mapping();
    let dim: usize = self.dim as usize;

    let pos: &Vec<f32> = &self.pos;
    let mut new_pos: Vec<f32> = Vec::with_capacity(new_len * dim);
    
    for i0 in mapping.iter() {
      for d in 0..dim {
        new_pos.push(pos[i0 * dim + d]);
      }
    }
    println!("Coarsening, new |V| = {}", new_len);
    let g1_pos = GraphPos::new(Rc::clone(&g1), new_pos, dim as u8)?;

    Ok((g1, link, g1_pos))
  }


  fn normalize2d(&self) -> Vec<f32> {
    let dim = self.dim as usize;
    let size: usize = self.pos.len() / dim;
    let d2: usize = 2;
    let mut res: Vec<f32> = Vec::with_capacity(size * d2);
    for i in 0..size {
      res.push(self.pos[i * dim + 0]);
      res.push(self.pos[i * dim + 1]);
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

  pub fn draw_to_img(&self, img: &mut Img) {
    let d2: usize = 2;
    let pos_2 = self.normalize2d();
    
    for (i, nexts) in self.g.adj.iter().enumerate() {
      let item0 = &pos_2[(i * d2)..(i * d2 + d2)];
      for j in nexts.iter() {
        let item1 = &pos_2[(j * d2)..(j * d2 + d2)];
        img.draw_line_f(item0[0], item0[1], item1[0], item1[1]);
      }
    }
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
        state.serialize_field("dim", &self.dim)?;
        state.end()
    }
}