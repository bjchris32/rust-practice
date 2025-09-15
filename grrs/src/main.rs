use clap::Parser;
use std::io::BufRead;
use anyhow::{Context, Result};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf
}

// dyn means dynamic dispatch.
// dyn Error : “some type that implements the Error trait.” and “I don’t care which concrete error it is, just treat it like an Error.”
// Box<dyn Error> : “Box is a heap pointer to some type that implements Error, but we don’t know its size or exact type at compile time.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let file = std::fs::File::open(&args.path)
        .with_context(|| format!("could not read file `{}`", &args.path.display()))?;
    let reader = std::io::BufReader::new(file);
    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
