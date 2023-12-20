use std::process;
pub mod entry;

use clap::{Command, ArgMatches};

fn cli() -> Command {
  Command::new("graph")
  .author("Zhifeng Wang, zhifeng5@illinois.edu")
  .about("A command line tool for Graph Visualization")
  .subcommand_required(true)
  .allow_external_subcommands(true)
  .subcommand(
    entry::coarsen_cli()
  )
  .subcommand(
    entry::draw_cli()
  )
}

fn run(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {

  let matches: ArgMatches = cli().get_matches_from(args);
  
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Err(e) = run(args) {
        eprintln!("Sorry, there is an error: {e}");
        process::exit(1);
    }
}