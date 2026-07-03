//! CLI integration tests for `vp-reference verify`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;
use tempfile::TempDir;
use vp_reference_cli::{run_verify, OutputFormat, VerifyOptions, EXIT_SUCCESS, EXIT_USER_ERROR};

fn bin_path() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_vp-reference"))
}

fn examples_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples")
}

fn write_json(dir: &Path, name: &str, contents: &str) -> PathBuf {
    let path = dir.join(name);
    fs::write(&path, contents).expect("write fixture");
    path
}

fn run_verify_cli(claim: &Path, evidence: &Path, format: &str) -> (i32, String, String) {
    let output = Command::new(bin_path())
        .args([
            "verify",
            "--claim",
            claim.to_str().expect("claim path"),
            "--evidence",
            evidence.to_str().expect("evidence path"),
            "--format",
            format,
        ])
        .output()
        .expect("run cli");

    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");
    (output.status.code().unwrap_or(-1), stdout, stderr)
}

#[test]
fn normalized_text_example_is_satisfied() {
    let claim = examples_dir().join("claim.normalized_text.json");
    let evidence = examples_dir().join("evidence.normalized_text.json");

    let output = run_verify(&VerifyOptions::new(claim, evidence, OutputFormat::Human))
        .expect("normalized_text verify");

    assert!(output.rendered().contains("✓ satisfied claim-001"));
    assert!(output.rendered().contains("Reason:"));
}

#[test]
fn body_equality_is_satisfied() {
    let dir = TempDir::new().expect("tempdir");
    let claim = write_json(
        dir.path(),
        "claim.json",
        r#"{
  "claim_id": "claim-001",
  "subject": "example-subject",
  "assertion": {
    "assertion_type": "body_equality",
    "body": "alpha"
  }
}"#,
    );
    let evidence = write_json(
        dir.path(),
        "evidence.json",
        r#"{
  "evidence_id": "evidence-001",
  "claim_id": "claim-001",
  "evidence_type": "document",
  "content": {
    "content_type": "document",
    "body": "alpha"
  }
}"#,
    );

    let output = run_verify(&VerifyOptions::new(claim, evidence, OutputFormat::Human))
        .expect("body_equality verify");

    assert!(output.rendered().contains("✓ satisfied claim-001"));
}

#[test]
fn normalized_text_case_mismatch_is_not_satisfied() {
    let dir = TempDir::new().expect("tempdir");
    let claim = write_json(
        dir.path(),
        "claim.json",
        r#"{
  "claim_id": "claim-001",
  "subject": "example-subject",
  "assertion": {
    "assertion_type": "normalized_text",
    "body": "Hello"
  }
}"#,
    );
    let evidence = write_json(
        dir.path(),
        "evidence.json",
        r#"{
  "evidence_id": "evidence-001",
  "claim_id": "claim-001",
  "evidence_type": "document",
  "content": {
    "content_type": "text/plain",
    "body": "hello"
  }
}"#,
    );

    let output = run_verify(&VerifyOptions::new(claim, evidence, OutputFormat::Human))
        .expect("case mismatch verify");

    assert!(output.rendered().contains("✗ not_satisfied claim-001"));
}

#[test]
fn whitespace_only_evidence_is_indeterminate() {
    let dir = TempDir::new().expect("tempdir");
    let claim = write_json(
        dir.path(),
        "claim.json",
        r#"{
  "claim_id": "claim-001",
  "subject": "example-subject",
  "assertion": {
    "assertion_type": "normalized_text",
    "body": "Hello"
  }
}"#,
    );
    let evidence = write_json(
        dir.path(),
        "evidence.json",
        r#"{
  "evidence_id": "evidence-001",
  "claim_id": "claim-001",
  "evidence_type": "document",
  "content": {
    "content_type": "text/plain",
    "body": "     "
  }
}"#,
    );

    let output = run_verify(&VerifyOptions::new(claim, evidence, OutputFormat::Human))
        .expect("whitespace-only verify");

    assert!(output.rendered().contains("? indeterminate claim-001"));
}

#[test]
fn malformed_claim_file_returns_user_error() {
    let dir = TempDir::new().expect("tempdir");
    let claim = write_json(
        dir.path(),
        "claim.json",
        r#"{
  "claim_id": "claim-001",
  "subject": "example-subject"
}"#,
    );
    let evidence = write_json(
        dir.path(),
        "evidence.json",
        r#"{
  "evidence_id": "evidence-001",
  "claim_id": "claim-001",
  "evidence_type": "document",
  "content": {
    "content_type": "text/plain",
    "body": "alpha"
  }
}"#,
    );

    let (code, _stdout, stderr) = run_verify_cli(&claim, &evidence, "human");

    assert_eq!(code, EXIT_USER_ERROR);
    assert!(stderr.contains("error:"));
}

#[test]
fn json_format_output_is_valid() {
    let claim = examples_dir().join("claim.normalized_text.json");
    let evidence = examples_dir().join("evidence.normalized_text.json");

    let (code, stdout, stderr) = run_verify_cli(&claim, &evidence, "json");

    assert_eq!(code, EXIT_SUCCESS);
    assert!(stderr.is_empty());

    let value: Value = serde_json::from_str(&stdout).expect("valid json");
    assert_eq!(value["claim_id"], "claim-001");
    assert_eq!(value["outcome"], "satisfied");
    assert!(value["reason"].is_string());
    assert!(value["trace"].is_array());
    assert!(!value["trace"].as_array().expect("trace array").is_empty());
}

#[test]
fn verify_cli_exits_zero_on_success() {
    let claim = examples_dir().join("claim.normalized_text.json");
    let evidence = examples_dir().join("evidence.normalized_text.json");

    let (code, stdout, stderr) = run_verify_cli(&claim, &evidence, "human");

    assert_eq!(code, EXIT_SUCCESS);
    assert!(stdout.contains("✓ satisfied claim-001"));
    assert!(stderr.is_empty());
}
