use grrs_bj::*;
use indicatif::ProgressBar;
use assert_cmd::cargo::*; // Import cargo_bin_cmd! macro and methods
use predicates::prelude::*; // Used for writing assertions
use assert_fs::fixture::FileWriteStr;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("grrs-bj");

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[test]
fn test_find_match() {
    let mut result = Vec::new();
    let _ = find_match("lorem ipsum", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

#[test]
fn test_find_no_match() {
    let mut result = Vec::new();
    let _ = find_match("lorem ipsum", "abc", &mut result);
    assert_eq!(result, b"");
}

#[test]
fn test_find_matches() {
    use std::io::Cursor;
    let input = b"lorem ipsum\ndolor sit amet\nlorem dolor\nconsectetur adipiscing";
    let reader = std::io::BufReader::new(Cursor::new(input));
    let pb = ProgressBar::new(4); // dummy
    let mut writer = Vec::new();
    let result = find_matches(reader, &pb, "lorem", &mut writer);

    assert!(result.is_ok());
    let output = String::from_utf8(writer).unwrap();
    assert!(output.contains("lorem ipsum"));
    assert!(output.contains("lorem dolor"));
    assert!(!output.contains("dolor sit amet"));
    assert!(!output.contains("consectetur adipiscing"));
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = cargo_bin_cmd!("grrs-bj");
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A test\nAnother test"));

    Ok(())
}