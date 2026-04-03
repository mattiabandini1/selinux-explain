use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn test_pipe_input_httpd_read() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_selinux-explain"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn process");

    let mock_log = "type=AVC msg=audit(1612345678.123:456): avc:  denied  { read } for  pid=1234 comm=\"nginx\" name=\"index.html\" scontext=system_u:system_r:httpd_t:s0 tcontext=unconfined_u:object_r:user_home_t:s0 tclass=file\n";

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin.write_all(mock_log.as_bytes()).expect("Failed to write to stdin");
    });

    let output = child.wait_with_output().expect("Failed to read stdout");
    let output_str = String::from_utf8_lossy(&output.stdout);

    // These strings must match what's actually in rules.toml for httpd_t + read + file
    assert!(output_str.contains("wrong SELinux label"), "Expected rules.toml suggestion, got: {}", output_str);
    assert!(output_str.contains("restorecon"), "Expected fix command, got: {}", output_str);
}

#[test]
fn test_pipe_input_no_match() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_selinux-explain"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn process");

    // unknown_t is not in rules.toml — should hit the hardcoded fallback
    let mock_log = "type=AVC msg=audit(1612345678.123:456): avc:  denied  { read } for  pid=1234 comm=\"unknown\" name=\"somefile\" dev=\"sda1\" ino=12345 scontext=system_u:system_r:unknown_t:s0 tcontext=unconfined_u:object_r:user_home_t:s0 tclass=file\n";

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin.write_all(mock_log.as_bytes()).expect("Failed to write to stdin");
    });

    let output = child.wait_with_output().expect("Failed to read stdout");
    let output_str = String::from_utf8_lossy(&output.stdout);

    // Should hit the hardcoded fallback
    assert!(output_str.contains("No specific advice"), "Expected fallback, got: {}", output_str);
}

#[test]
fn test_pipe_no_avc_input() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_selinux-explain"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn process");

    let mock_log = "This is just a random systemd log line, nothing to see here.\n";

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin.write_all(mock_log.as_bytes()).expect("Failed to write to stdin");
    });

    let output = child.wait_with_output().expect("Failed to read stdout");
    let output_str = String::from_utf8_lossy(&output.stdout);

    assert!(output_str.contains("No SELinux denials found"), "Expected no-match message, got: {}", output_str);
}
