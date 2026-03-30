use regex::Regex;

/// Struct that holds the relevant information extracted from a SELinux log.
/// #[derive(Debug, PartialEq)] allows us to print the struct and compare it in tests.
#[derive(Debug, PartialEq)]
pub struct AvcData {
    pub process: String,
    pub action: String,
    pub target: String,
    pub scontext: String,
    pub tcontext: String,
    pub tclass: String,
}

/// Parses a single line of log.
/// Returns Some(AvcData) if it's a valid SELinux AVC denial.
/// Returns None if the line does not match the expected format.
pub fn parse_avc_log(log_line: &str) -> Option<AvcData> {
    let re = Regex::new(r#"denied\s*\{\s*(.*?)\s*\}.*?comm="(.*?)".*?name="(.*?)".*?scontext=(\S+).*?tcontext=(\S+).*?tclass=(\S+)"#).ok()?;

    if let Some(captures) = re.captures(log_line) {
        let action = captures.get(1)?.as_str().to_string();
        let process = captures.get(2)?.as_str().to_string();
        let target = captures.get(3)?.as_str().to_string();
        let scontext = captures.get(4)?.as_str().to_string();
        let tcontext = captures.get(5)?.as_str().to_string();
        let tclass = captures.get(6)?.as_str().to_string();

        return Some(AvcData {
            process,
            action,
            target,
            scontext,
            tcontext,
            tclass,
        });
    }
    None
}

/// --- UNIT TESTS ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_avc_log() {
        // A raw string simulating a real SELinux denial
        let raw_log = r#"type=AVC msg=audit(1612345678.123:456): avc:  denied  { read } for  pid=1234 comm="nginx" name="index.html" dev="sda1" ino=12345 scontext=system_u:system_r:httpd_t:s0 tcontext=unconfined_u:object_r:user_home_t:s0 tclass=file permissive=0"#;
        
        // The expected struct we want our parser to build
        let expected = AvcData {
            process: "nginx".to_string(),
            action: "read".to_string(),
            target: "index.html".to_string(),
            scontext: "system_u:system_r:httpd_t:s0".to_string(),
            tcontext: "unconfined_u:object_r:user_home_t:s0".to_string(),
            tclass: "file".to_string(),
        };

        assert_eq!(parse_avc_log(raw_log), Some(expected));
    }

    #[test]
    fn test_parse_invalid_log() {
        let bad_log = "Just a random systemd log, nothing to see here.";
        assert_eq!(parse_avc_log(bad_log), None);
    }
}
