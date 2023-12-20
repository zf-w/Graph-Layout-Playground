use std::path::PathBuf;
use std::{fs, rc::Rc};

use graph::Graph;
use graph::graph::GraphPos;

use clap::Arg;
use clap::{ArgMatches, Command};

const GRAPH_JSON: &str = "graph_json";

pub fn coarsen_cli() -> Command {
  Command::new("coarsen")
    .about("Coarsen a graph")
    .arg(Arg::new(GRAPH_JSON)
      .help("The path to the JSON file of your input graph")
      .value_parser(clap::value_parser!(PathBuf))
      .required(true))
    .arg(
      clap::Arg::new("out")
      .short('o')
      .long("output-name")
      .help("The name of the output graph json")
      .value_parser(clap::value_parser!(PathBuf))
      .default_value("output.json")
    )
    .arg(Arg::new("depth")
      .short('d')
      .long("depth")
      .help("The maximum level of coarsening iteration")
      .default_value("1"))
}

#[derive(PartialEq, Debug)]
struct Config<'a> {
  file: &'a PathBuf,
  out: &'a PathBuf,
  level: u16,
}

impl<'a> Config<'a> {
  pub fn new(sub_matches: &'a ArgMatches) -> Result<Self, &'static str> {
    let file = sub_matches.get_one::<PathBuf>(GRAPH_JSON).expect("Expecting a JSON graph file");
    let out = sub_matches.get_one::<PathBuf>("out").expect("Should have a default value");

    let depth = 
    match sub_matches.get_one::<String>("depth") {
      Some(str) => {
        str.parse::<u16>().unwrap_or(1)
      }
      _ => 1
    };

    Ok(Config { file, level: depth, out })
  }
}

fn coarsen_with_pos(mut g_pos: GraphPos, level: u16) -> Result<GraphPos, Box<dyn std::error::Error>> {
  let mut i: u16 = 0;
  while i < level {
    (_, _, g_pos) = g_pos.coarsen()?;
    i += 1;
  }
  Ok(g_pos)
}

pub fn run_coarsen(args: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
  let config = Config::new(args)?;
  let contents: String = fs::read_to_string(config.file)?;
  
  let graph::Input {indices, position, dim} = serde_json::from_str(&contents)?;

  let g0: Rc<Graph> = Graph::from_edge_list(indices);
  
  if position.is_some() && dim.is_some() {
    let g_pos = GraphPos::new(g0, position.unwrap(), dim.unwrap())?;
    let g_pos1 = coarsen_with_pos(g_pos, config.level)?;
    let to_write = serde_json::to_string(&g_pos1)?;
    fs::write(config.out, to_write)?;
  }

  Ok(())
}



#[cfg(test)]
mod tests {
  use super::*;
  use clap::ArgMatches;
  
  fn get_arg_matches(arg_vec: Vec<String>) -> ArgMatches {
    coarsen_cli().get_matches_from(arg_vec)
  }

  #[test]
  fn config_json_default_depth() {
    let arg_vec = vec![String::from("test"), String::from("data/data.json")];
    let path = PathBuf::from("data/data.json");
    let out_exp = PathBuf::from("output.json");
    let expected = Config {
      file: &path,
      out: &out_exp,
      level: 1
    };
    let arg_matches = get_arg_matches(arg_vec);
    let res = 
      Config::new(&arg_matches);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), expected);
  }

   #[test]
  fn config_json_depth_short() {
    let arg_vec = vec![
      String::from("test"),
      String::from("data/data.json"),
      String::from("-d"),
      String::from("2")];
    let path = PathBuf::from(r"data/data.json");
    let out_exp = PathBuf::from(r"output.json");
    let expected = Config {
      file: &path,
      out: &out_exp,
      level: 2
    };
    let arg_matches = get_arg_matches(arg_vec);
    let res = 
      Config::new(&arg_matches);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), expected);
  }
}