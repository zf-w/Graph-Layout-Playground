use std::{env, process};
use graph_layout::run;


fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(e) = run(&args) {
        eprintln!("Sorry, there is an error: {e}");
        process::exit(1);
    }
}