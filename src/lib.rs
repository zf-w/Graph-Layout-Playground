pub mod graph;
pub mod img;
pub mod entry;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Input {
  pub indices: Vec<usize>,
  pub position: Option<Vec<f32>>,
  pub dim: Option<u8>
}

pub fn run(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
  
  if args.len() < 2 {
    return Err("Not enough arguments?".into());
  }

  match args[1].as_str() {
    "coarsen" => {
      entry::run_coarsen(args)?;
    },
    "draw" => {
      entry::run_draw(args)?;
    },
    _ => return Err("Command not recoginzed?".into()),
  }
  

  Ok(())
}