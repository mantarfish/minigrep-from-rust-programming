use std::env;
// use std::io::prelude::*;
use std::process;
extern crate minigrep;
use minigrep::Config;
use minigrep::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application Error: {}", e);
        process::exit(1);
    }
}
