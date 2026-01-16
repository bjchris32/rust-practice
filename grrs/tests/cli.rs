use grrs::*;
use indicatif::ProgressBar;

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
