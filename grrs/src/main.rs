use clap::Parser;
use std::io::BufRead;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf
}

fn main() {
    let args = Cli::parse();
    let file = std::fs::File::open(&args.path).expect("could not open file");
    let reader = std::io::BufReader::new(file);
    for line_result in reader.lines() {
        let line = line_result.expect("could not read line");
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}
