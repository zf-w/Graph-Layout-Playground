use std::{fs, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::graph::Graph;
use crate::graph::GraphPos;
use crate::img::Img;

struct Config<'a> {
  file: &'a str,
  width: u32,
}

impl<'a> Config<'a> {
  pub fn new(args: &'a [String]) -> Result<Self, &'static str> {
    let len: usize = args.len();
    if len < 3 {
      return Err("Not enough arguments for command coarsen? Did you include the graph file?");
    }

    let width: u32 = if let Ok(num) = &args[3].parse::<u32>() {
      *num
    } else {
      1080
    };

    Ok(Config { file: &args[2],  width })
  }
}

#[derive(Deserialize, Serialize)]
struct Input {
  pub indices: Vec<usize>,
  pub position: Vec<f32>,
  pub dim: u8
}

pub fn run_draw(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
  let config = Config::new(args)?;
  let contents: String = fs::read_to_string(config.file)?;

  let Input {indices, position, dim} = serde_json::from_str(&contents)?;

  let g: Rc<Graph> = Graph::from_edge_list(indices);
  let g_pos: GraphPos = GraphPos::new(g, position, dim)?;

  let mut img = Img::new(config.width, config.width);
  g_pos.draw_to_img(&mut img, true);
  img.save("data/output.png")?;

  Ok(())
}