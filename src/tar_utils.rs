use regex::Regex;

pub fn line_find_bytes_read_or_written(line: &str) -> Option<u64> {
    let re = Regex::new(r"\s\d+\s").unwrap();
    let find = re.captures(line)?;
    let result = find.get(0)?.as_str().parse::<u64>().ok()?;
    Some(result)
}

pub fn line_find_speed(line: &str) -> Option<&str> {
    let re = Regex::new(r"\s\(\d*\w*, (\d+\w+/s)\)$").unwrap();
    let find = re.captures(line)?;
    let result = find.get(0)?.as_str();

    Some(result)
}
