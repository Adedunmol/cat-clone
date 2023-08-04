use clap::Parser;
use std::{path, process, fs::File, error::Error, io::{self, BufReader, prelude::*, BufWriter, Stdout}};
use anyhow::{Context, Result};

/// Display the content of a file
#[derive(Parser)]
pub struct Cli {
    /// Path to the file 
    path: path::PathBuf
}

pub fn run(args: &Cli) -> Result<(), Box<dyn Error>> {
    let handle = buffered_stdout();

    let _ = read_file(&args, handle)?;


    Ok(())
}

fn read_file(args: &Cli, mut writer: impl std::io::Write) -> Result<(), Box<dyn Error>> {

    let file = File::open(&args.path)?;
    let mut reader = BufReader::new(file);

    let mut content = String::new();

    while reader.read_line(&mut content)
                .with_context(|| format!("could not read file: `{}`", &args.path.to_string_lossy()))
                .unwrap() > 0 
    {
        writeln!(writer, "{}", content.trim()).unwrap_or_else(|err| {
            eprintln!("Couldn't write to the terminal, {}", err);
            process::exit(1);
        });
        content.clear();
        
    }

    Ok(())
}

fn buffered_stdout() -> BufWriter<Stdout> {
    let stdout = io::stdout();
    let handle = io::BufWriter::new(stdout);

    handle
}
