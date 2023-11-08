use std::{env, process};
use graph_layout::{Config, run};


fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args).unwrap_or_else(|err: &str| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprint!("Application error: {e}");
        process::exit(1);
    }
}