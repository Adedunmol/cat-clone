use clap::Parser;
use std::{path, fs::File, error::Error, io::{self, Write, BufReader, prelude::*, BufWriter, Stdout}};
use anyhow::{Context, Result};

/// Display the content of a file
#[derive(Parser)]
pub struct Cli {
    /// Path to the file 
    path: path::PathBuf
}

pub fn run(args: &Cli) -> Result<(), Box<dyn Error>> {

    let content = read_file(&args)?;


    Ok(())
}

fn read_file(args: &Cli) -> Result<(), Box<dyn Error>> {
    let mut handle = buffer_stdout();

    let file = File::open(&args.path)?;
    let mut reader = BufReader::new(file);

    let mut content = String::new();

    while reader.read_line(&mut content)
                .with_context(|| format!("could not read file `{}`", &args.path.to_string_lossy()))
                .unwrap() > 0 
    {
        writeln!(handle, "{}", content.trim());
        content.clear();
        
    }

    Ok(())
}

fn buffer_stdout() -> BufWriter<Stdout> {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    handle
}