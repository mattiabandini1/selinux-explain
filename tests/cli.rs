use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn test_pipe_input() {
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

    assert!(output_str.contains("The web server is trying to read a file or directory"));
    assert!(output_str.contains("index.html"));
    assert!(output_str.contains("sudo restorecon -Rv"));
}
