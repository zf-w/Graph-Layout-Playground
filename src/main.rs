use std::process;
pub mod entry;

use clap::{Command, ArgMatches, Arg};

fn cli() -> Command {
  Command::new("graph")
  .about("A tool for Graph Visualization")
  .subcommand_required(true)
  .allow_external_subcommands(true)
  .subcommand(
    entry::coarsen_cli()
    // .arg(arg!(<file> "The JSON file of a graph").required(true))
    // .arg(arg!(<depth> "The maximum level of coarsening iteration"))
  )
  .subcommand(
    Command::new("draw")
    .about("Draw a graph")
    .arg(
      Arg::new("json")
      .help("The path to the JSON file of your input graph")
      .required(true)
      // clap::arg!(<file> "The JSON file of a graph").required(true))
    )
    .arg(
      Arg::new("width")
      .short('w')
      .long("width")
      .help("The width of the output image")
      .default_value("1080")
    )
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