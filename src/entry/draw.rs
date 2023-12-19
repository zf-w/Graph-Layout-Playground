use graph::img::{Drawable, Svg};
use std::{fs, rc::Rc};

use graph::Graph;
use graph::graph::GraphPos;

use clap::ArgMatches;

pub fn draw_cli() -> clap::Command {
  clap::Command::new("draw")
    .about("Draw a graph")
    .arg(
      clap::Arg::new("json")
      .help("The path to the JSON file of your input graph")
      .required(true)
      // clap::arg!(<file> "The JSON file of a graph").required(true))
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

use graph::Input;

pub fn run_draw(sub_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
  let config = Config::new(sub_matches)?;
  let contents: String = fs::read_to_string(config.file)?;

  let Input {indices, position, dim} = serde_json::from_str(&contents)?;

  let g: Rc<Graph> = Graph::from_edge_list(indices);
  let g_pos: GraphPos = GraphPos::new(g, position.expect("Should have position"), dim.expect("Should have dim"))?;

  let mut img = Svg::new(config.width, config.width);//Img::new(config.width, config.width);
  g_pos.draw_to_img(&mut img, true);
  img.save("data/output.svg")?;

  Ok(())
}