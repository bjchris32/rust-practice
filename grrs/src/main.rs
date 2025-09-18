use clap::Parser;
use std::io::BufRead;
use anyhow::{Context, Result};
use std::io::{self, Write};
use indicatif::ProgressBar;
use std::time::Duration; // optional: sleep 50 ms to observe the progress bar
use std::thread; // optional: sleep 50 ms to observe the progress bar

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf
}

fn find_match(line: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<(), Box<dyn std::error::Error>> {
    if line.contains(pattern) {
        writeln!(writer, "{}", line)?;
        thread::sleep(Duration::from_millis(50)); // optional: sleep 50 ms to observe the progress bar
    }

    Ok(())
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

    let pb = ProgressBar::new(100);

    for line_result in reader.lines() {
        let line = line_result?;
        // TODO: handle the returned value from find_match
        find_match(&line, &args.pattern, &mut writer);
        pb.inc(1);
    }

    pb.finish_with_message("done");

    Ok(())
}

fn answer() -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_answer_validity() {
        assert_eq!(answer(), 42);
    }

    #[test]
    fn find_a_match() {
        let mut result = Vec::new();
        // TODO: handle the returned value from find_match
        find_match("lorem ipsum", "lorem", &mut result);
        assert_eq!(result, b"lorem ipsum\n");
    }
}
