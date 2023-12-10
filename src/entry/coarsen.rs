use std::{fs, rc::Rc};

use crate::graph::Graph;
use crate::graph::GraphPos;

use clap::ArgMatches;

struct Config<'a> {
  file: &'a str,
  level: u16,
}

impl<'a> Config<'a> {
  pub fn new(sub_matches: &'a ArgMatches) -> Result<Self, &'static str> {
    let file = sub_matches.get_one::<String>("file").expect("required");

    let depth = 
    match sub_matches.get_one::<String>("depth") {
      Some(str) => {
        str.parse::<u16>().unwrap_or(1)
      }
      _ => 1
    };

    Ok(Config { file, level: depth })
  }
}

fn coarsen_with_pos(mut g_pos: GraphPos, level: u16) -> Result<(), Box<dyn std::error::Error>> {
  let mut i: u16 = 0;
  while i < level {
    (_, _, g_pos) = g_pos.coarsen()?;
    i += 1;
  }
  let g_str = serde_json::to_string(&g_pos)?;
  fs::write("data/output.graph.json", &g_str)?;
  // let mut img = Img::new(1080, 1080);
  // g1_pos.draw_to_img(&mut img);
  // img.save("data/output.png")?;
  Ok(())
}

pub fn run_coarsen(args: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
  let config = Config::new(args)?;
  let contents: String = fs::read_to_string(config.file)?;

  let crate::Input {indices, position, dim} = serde_json::from_str(&contents)?;

  let g0: Rc<Graph> = Graph::from_edge_list(indices);
  
  if position.is_some() && dim.is_some() {
    let g_pos = GraphPos::new(g0, position.unwrap(), dim.unwrap())?;
    coarsen_with_pos(g_pos, config.level)?;
  }

  Ok(())
}