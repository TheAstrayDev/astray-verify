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
