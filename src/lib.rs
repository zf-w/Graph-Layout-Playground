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

use clap::{arg, Command, ArgMatches};

fn cli() -> Command {
  Command::new("graph")
  .about("A tool for Graph Visualization")
  .subcommand_required(true)
  .allow_external_subcommands(true)
  .subcommand(
    Command::new("coarsen")
    .about("graph coarsen")
    .arg(arg!(<file> "The JSON file of a graph").required(true))
    .arg(arg!(<depth> "The maximum level of coarsening iteration"))
    
  )
  .subcommand(
    Command::new("draw")
    .about("Draw a graph")
    .arg(arg!(<file> "The JSON file of a graph").required(true))
    .arg(
      arg!(-w -- width <width> "The width of the output image")
      .default_value("1080")
    )
    
  )
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {

  let matches: ArgMatches = cli().get_matches();

  match matches.subcommand() {
    Some(("coarsen", sub_matches)) => {
      entry::run_coarsen(sub_matches)?;
    },
    Some(("draw", sub_matches)) => {
      entry::run_draw(sub_matches)?;
    },
    _ => return Err("Command not recoginzed?".into()),
  }
  

  Ok(())
}