use std::process;
use graph::run;


fn main() {
    if let Err(e) = run() {
        eprintln!("Sorry, there is an error: {e}");
        process::exit(1);
    }
}