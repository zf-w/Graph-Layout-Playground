use graph::img::{Drawable, Svg, Img};
use std::path::PathBuf;
use std::{fs, rc::Rc};

use graph::Graph;
use graph::graph::GraphPos;

pub fn draw_cli() -> clap::Command {
  clap::Command::new("draw")
    .about("Draw a graph")
    .arg(
      clap::Arg::new("JSON")
      .help("The path to the JSON file of your input graph")
      .value_parser(clap::value_parser!(PathBuf))
      .required(true)
      // clap::arg!(<file> "The JSON file of a graph").required(true))
    )
    .arg(
      clap::Arg::new("out")
      .short('o')
      .long("output-name")
      .help("The name of the output image")
      .value_parser(clap::value_parser!(PathBuf))
      .default_value("output.svg")
    )
    .arg(
      clap::Arg::new("mass")
      .short('m')
      .long("mass-center")
      .action(clap::ArgAction::SetTrue)
    )
    .arg(
      clap::Arg::new("width")
      .short('w')
      .long("width")
      .help("The width of the output image")
      .default_value("1080")
    )
}
struct Config<'a> {
  file: &'a PathBuf,
  out: &'a PathBuf,
  width: u32,
  draw_full: bool
}

impl<'a> Config<'a> {
  pub fn new(sub_matches: &'a clap::ArgMatches) -> Result<Self, &'static str> {
    let file = sub_matches.get_one::<PathBuf>("JSON").expect("required");
    let out = sub_matches.get_one::<PathBuf>("out").expect("Should have a default value");
    let draw_full = !sub_matches.get_flag("mass");
    let width: u32 = 
    match sub_matches.get_one::<String>("width") {
      Some(str) => {
        str.parse::<u32>().unwrap_or(1080)
      }
      _ => 1080
    };

    Ok(Config { file, out,  width, draw_full })
  }
}

use graph::Input;

pub fn run_draw(sub_matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
  let config = Config::new(sub_matches)?;
  let contents: String = fs::read_to_string(config.file)?;

  let Input {indices, position, dim} = serde_json::from_str(&contents)?;
  if position.is_none() || dim.is_none() {
    return Err("Sorry, the graph file you provided doesn't contain necessary information.".into());
  }


  let g: Rc<Graph> = Graph::from_edge_list(indices);
  let g_pos: GraphPos = GraphPos::new(g, position.expect("Should have position"), dim.expect("Should have dim"))?;
  match config.out.extension() {
    Some(e) => {
      match e.to_str() {
        Some("png") => {
          let mut img = Img::new(config.width, config.width);
          g_pos.draw_to_img(&mut img, config.draw_full);
          img.save(config.out)?;
        },
        Some("svg") => {
          let mut img = Svg::new(config.width, config.width);//Img::new(config.width, config.width);
        g_pos.draw_to_img(&mut img, config.draw_full);
        img.save(config.out)?;
        },
        _ => {
          return Err("Sorry, the input image extension is currently not supported.".into())
        }
      }
    },
    _ => {
      return Err("It seems you are not putting in a valid filename with proper file extensions.".into())
    }
  }
  
  

  Ok(())
}