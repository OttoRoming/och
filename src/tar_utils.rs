use regex::Regex;

/// Finds the number of bytes processed from a line when using
/// tar with --checkpoint and --checkpoint-action=totals
///
/// bytes processed refers to either:
/// - bytes read during extraction
/// - bytes written during archive creation
pub fn line_find_bytes_processed(line: &str) -> Option<u64> {
    let re = Regex::new(r"\s\d+\s").unwrap();
    let find = re.captures(line)?;
    let result = find.get(0)?.as_str().parse::<u64>().ok()?;
    Some(result)
}

/// Finds the operation speed of tar from a line when using
/// tar with --checkpoint and --checkpoint-action=totals
pub fn line_find_speed(line: &str) -> Option<&str> {
    let re = Regex::new(r"\s\(\d*\w*, (\d+\w+/s)\)$").unwrap();
    let find = re.captures(line)?;
    let result = find.get(0)?.as_str();

    Some(result)
}
