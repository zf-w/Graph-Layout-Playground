use std::{fs, rc::Rc};

use serde::{Deserialize, Serialize};

use graph::Graph;
use graph::graph::GraphPos;
use graph::img::Img;

struct Config<'a> {
  file: &'a str,
  width: u32,
}

impl<'a> Config<'a> {
  pub fn new(sub_matches: &'a ArgMatches) -> Result<Self, &'static str> {
    let file = sub_matches.get_one::<String>("json").expect("required");

    let width: u32 = 
    match sub_matches.get_one::<String>("width") {
      Some(str) => {
        str.parse::<u32>().unwrap_or(1)
      }
      _ => 1080
    };

    Ok(Config { file,  width })
  }
}

#[derive(Deserialize, Serialize)]
struct Input {
  pub indices: Vec<usize>,
  pub position: Vec<f32>,
  pub dim: u8
}

use clap::ArgMatches;

pub fn run_draw(sub_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
  let config = Config::new(sub_matches)?;
  let contents: String = fs::read_to_string(config.file)?;

  let Input {indices, position, dim} = serde_json::from_str(&contents)?;

  let g: Rc<Graph> = Graph::from_edge_list(indices);
  let g_pos: GraphPos = GraphPos::new(g, position, dim)?;

  let mut img = Img::new(config.width, config.width);
  g_pos.draw_to_img(&mut img, true);
  img.save("data/output.png")?;

  Ok(())
}