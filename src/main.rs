use std::process;

use cat_clone::{Cli, run};
use clap::Parser;
use log::{info};


fn main() {
    env_logger::init();
    info!("Starting up");

    let args = Cli::parse();

    if let Err(error) = run(&args) {
        eprintln!("could not open the file: {}", error);
        process::exit(1);
    };
}
