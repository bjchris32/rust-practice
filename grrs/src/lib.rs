use std::io::BufRead;
use indicatif::ProgressBar;
use std::time::Duration; // optional: sleep 50 ms to observe the progress bar
use std::thread; // optional: sleep 50 ms to observe the progress bar

pub fn find_match(line: &str, pattern: &str, writer: &mut impl std::io::Write) -> Result<(), Box<dyn std::error::Error>> {
    if line.contains(pattern) {
        writeln!(writer, "{}", line)?;
        thread::sleep(Duration::from_millis(50)); // optional: sleep 50 ms to observe the progress bar
    }

    Ok(())
}

pub fn find_matches<R: std::io::Read>(reader: std::io::BufReader<R>, pb: &ProgressBar, pattern: &str, mut writer: impl std::io::Write) -> Result<(), Box<dyn std::error::Error>> {
    for line_result in reader.lines() {
        let line = line_result?;
        find_match(&line, pattern, &mut writer)?;
        pb.inc(1);
    }
    Ok(())
}
