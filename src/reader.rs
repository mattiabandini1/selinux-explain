use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Reads the specified log file line by line and returns the very last SELinux denial found.
/// Returns an io::Error if the file cannot be opened (e.g., missing permissions).
pub fn get_last_denial(file_path: &str) -> io::Result<Option<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut last_match: Option<String> = None;

    for line_result in reader.lines() {
        let line = line_result?;

        if line.contains("type=AVC") && line.contains("denied") {
            last_match = Some(line);
        }
    }

    Ok(last_match)
}
