use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn temporary_project() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock before epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("astray-verify-e2e-{}-{nonce}", std::process::id()))
}

#[test]
fn records_and_replays_a_stdio_contract() {
    let root = temporary_project();
    fs::create_dir_all(&root).expect("create temporary project");

    let executable = env!("CARGO_BIN_EXE_astray-verify");
    let server = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/echo_server.py");

    let record = Command::new(executable)
        .current_dir(&root)
        .args(["record", "--name", "echo", "--", "python3"])
        .arg(&server)
        .output()
        .expect("run record command");
    assert!(
        record.status.success(),
        "record failed: {}",
        String::from_utf8_lossy(&record.stderr)
    );
    assert!(root.join("fixtures/echo.mcp.json").is_file());

    let replay = Command::new(executable)
        .current_dir(&root)
        .arg("test")
        .output()
        .expect("run test command");
    assert!(
        replay.status.success(),
        "replay failed: {}",
        String::from_utf8_lossy(&replay.stderr)
    );
    assert!(String::from_utf8_lossy(&replay.stdout).contains("PASS  echo"));

    fs::remove_dir_all(&root).expect("remove temporary project");
}

#[test]
fn audits_a_server_and_writes_an_execution_log() {
    let root = temporary_project();
    fs::create_dir_all(&root).expect("create temporary project");
    let executable = env!("CARGO_BIN_EXE_astray-verify");
    let server = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/echo_server.py");

    let audit = Command::new(executable)
        .current_dir(&root)
        .args(["--json", "--log", "audit.jsonl", "audit", "--", "python3"])
        .arg(&server)
        .output()
        .expect("run audit command");
    assert!(
        audit.status.success(),
        "audit failed: {}",
        String::from_utf8_lossy(&audit.stderr)
    );
    let report = String::from_utf8_lossy(&audit.stdout);
    assert!(report.contains("weakest_link"));
    let log = fs::read_to_string(root.join("audit.jsonl")).expect("read audit log");
    assert!(log.contains("\"command\":\"audit\""));

    fs::remove_dir_all(&root).expect("remove temporary project");
}

#[test]
fn doctor_summarizes_a_fresh_project() {
    let root = temporary_project();
    fs::create_dir_all(&root).expect("create temporary project");
    let executable = env!("CARGO_BIN_EXE_astray-verify");

    let doctor = Command::new(executable)
        .current_dir(&root)
        .args(["--json", "doctor"])
        .output()
        .expect("run doctor command");
    let stdout = String::from_utf8_lossy(&doctor.stdout);
    assert!(
        stdout.contains("\"command\":\"doctor\""),
        "doctor JSON output missing: {stdout}"
    );

    fs::remove_dir_all(&root).expect("remove temporary project");
}
