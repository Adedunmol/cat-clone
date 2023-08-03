use std::process;

use cat_clone::{Cli, run};
use clap::Parser;


fn main() {
    let args = Cli::parse();

    if let Err(error) = run(&args) {
        eprintln!("{}", error);
        process::exit(1);
    };
}
