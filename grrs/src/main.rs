use clap::Parser;
use std::io::BufRead;
use anyhow::{Context, Result};
use std::io::{self, Write};


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

    let stdout = io::stdout(); // get the global stdout entity
    let handle = stdout.lock(); // lock implement the Write trait
    let mut writer = io::BufWriter::new(handle); // BufWriter accept anything with Write trait

    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(&args.pattern) {
            writeln!(writer, "{}", line)?;
        }
    }

    Ok(())
}
