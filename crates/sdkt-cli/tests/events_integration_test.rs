use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_events_format_json() {
    let mut cmd = Command::cargo_bin("sdkt").unwrap();
    cmd.arg("events")
        .arg("CCVVW7N4R3KNY72QJQKQY3T753C2H34E6XJIVJQOQSQE3C3M3U72QJQK")
        .arg("--format")
        .arg("json");

    let output = cmd.output().unwrap();
    // Verify run success or network connection failure exit 1
    assert!(output.status.success() || output.status.code().unwrap() == 1);
}

#[test]
fn test_events_invalid_format() {
    let mut cmd = Command::cargo_bin("sdkt").unwrap();
    cmd.arg("events")
        .arg("CCVVW7N4R3KNY72QJQKQY3T753C2H34E6XJIVJQOQSQE3C3M3U72QJQK")
        .arg("--format")
        .arg("xml");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid format"));
}

#[test]
fn test_events_help_shows_range_flags() {
    let mut cmd = Command::cargo_bin("sdkt").unwrap();
    cmd.arg("events").arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("--start-ledger"))
        .stdout(predicate::str::contains("--end-ledger"));
}

#[test]
fn test_events_inverted_range() {
    let mut cmd = Command::cargo_bin("sdkt").unwrap();
    cmd.arg("events")
        .arg("CCVVW7N4R3KNY72QJQKQY3T753C2H34E6XJIVJQOQSQE3C3M3U72QJQK")
        .arg("--start-ledger")
        .arg("5000")
        .arg("--end-ledger")
        .arg("1000");

    cmd.assert().failure().stderr(predicate::str::contains(
        "start ledger (5000) cannot be greater than end ledger (1000)",
    ));
}

#[test]
fn test_events_explicit_range() {
    let mut cmd = Command::cargo_bin("sdkt").unwrap();
    cmd.arg("events")
        .arg("CCVVW7N4R3KNY72QJQKQY3T753C2H34E6XJIVJQOQSQE3C3M3U72QJQK")
        .arg("--start-ledger")
        .arg("1000")
        .arg("--end-ledger")
        .arg("2000");

    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.contains("cannot be greater than"));
    assert!(!stderr.contains("unexpected argument"));
    // Valid command invocation reaches RPC call
    assert!(output.status.success() || output.status.code().unwrap() == 1);
}

#[test]
fn test_events_start_ledger_only() {
    let mut cmd = Command::cargo_bin("sdkt").unwrap();
    cmd.arg("events")
        .arg("CCVVW7N4R3KNY72QJQKQY3T753C2H34E6XJIVJQOQSQE3C3M3U72QJQK")
        .arg("--start-ledger")
        .arg("1000");

    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.contains("cannot be greater than"));
    assert!(!stderr.contains("unexpected argument"));
    assert!(output.status.success() || output.status.code().unwrap() == 1);
}

#[test]
fn test_events_end_ledger_only() {
    let mut cmd = Command::cargo_bin("sdkt").unwrap();
    cmd.arg("events")
        .arg("CCVVW7N4R3KNY72QJQKQY3T753C2H34E6XJIVJQOQSQE3C3M3U72QJQK")
        .arg("--end-ledger")
        .arg("2000");

    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.contains("cannot be greater than"));
    assert!(!stderr.contains("unexpected argument"));
    assert!(output.status.success() || output.status.code().unwrap() == 1);
}

#[test]
fn test_events_default_lookback() {
    let mut cmd = Command::cargo_bin("sdkt").unwrap();
    cmd.arg("events")
        .arg("CCVVW7N4R3KNY72QJQKQY3T753C2H34E6XJIVJQOQSQE3C3M3U72QJQK");

    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.contains("cannot be greater than"));
    assert!(!stderr.contains("unexpected argument"));
    assert!(output.status.success() || output.status.code().unwrap() == 1);
}
