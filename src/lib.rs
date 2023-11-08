pub mod graph;
pub mod img;

use std::{fs, rc::Rc};
use graph::pos::GraphPos;
use serde::{Deserialize, Serialize};

use crate::graph::Graph;
use crate::img::Img;

pub struct Config {
  pub file_path: String,
}

#[derive(Deserialize, Serialize)]
struct Input {
  pub indices: Vec<usize>,
  pub position: Option<Vec<f32>>,
  pub Dim: Option<u8>
}

impl Config {
  pub fn build(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 2 {
      return Err("Not enough arguments");
    }
    Ok(Config {file_path: args[1].clone()})
  }
}

fn run_coarsen_with_pos(g_pos: GraphPos, _level: u16) -> Result<(), Box<dyn std::error::Error>> {
  let (_, _, g1_pos) = g_pos.coarsen()?;
  let g1_str = serde_json::to_string(&g1_pos)?;
  fs::write("data/output.graph.json", &g1_str)?;
  let img = Img::new(1080, 1080);
  img.save("data/output.png");
  Ok(())
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
 
  let contents: String = fs::read_to_string(config.file_path)?;

  let Input {indices, position, Dim} = serde_json::from_str(&contents)?;

  let g0: Rc<Graph> = Graph::from_edge_list(indices);
  
  if position.is_some() && Dim.is_some() {
    let g_pos = GraphPos::new(g0, position.unwrap(), Dim.unwrap())?;
    run_coarsen_with_pos(g_pos, 1)?
  }

  Ok(())
}