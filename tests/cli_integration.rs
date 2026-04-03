// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6
//! CLI integration tests for wowasticker-cli binary.

use std::process::Command;

fn cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_wowasticker-cli"))
}

#[test]
fn cli_no_args_prints_help() {
    let out = cli().output().unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("USAGE"));
    assert!(stdout.contains("demo"));
    assert!(stdout.contains("govdocs"));
}

#[test]
fn cli_help_flag() {
    let out = cli().arg("--help").output().unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("USAGE"));
}

#[test]
fn cli_version_flag() {
    let out = cli().arg("--version").output().unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("wowasticker-cli"));
    assert!(stdout.contains("0.2.0"));
}

#[test]
fn cli_unknown_command_exits_nonzero() {
    let out = cli().arg("nonexistent").output().unwrap();
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("Unknown command"));
}

#[test]
fn cli_sbom_prints_spdx() {
    let out = cli().arg("--sbom").output().unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("SPDXVersion: SPDX-2.3"));
    assert!(stdout.contains("PackageName: wowasticker"));
    assert!(stdout.contains("Unlicense"));
}

#[test]
fn cli_govdocs_list() {
    let out = cli().arg("govdocs").output().unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("sbom"));
    assert!(stdout.contains("security"));
    assert!(stdout.contains("privacy"));
}

#[test]
fn cli_govdocs_security() {
    let out = cli().args(["govdocs", "security"]).output().unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(!stdout.is_empty());
}

#[test]
fn cli_govdocs_unknown() {
    let out = cli().args(["govdocs", "fake"]).output().unwrap();
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("Unknown govdoc"));
}

#[test]
fn cli_demo_runs_to_completion() {
    let out = cli().arg("demo").output().unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("WowaSticker Demo"));
    assert!(stdout.contains("Demo complete"));
    assert!(stdout.contains("Student:"));
    assert!(stdout.contains("Daily Report"));
}

#[test]
fn cli_sbom_lists_dependencies() {
    let out = cli().arg("--sbom").output().unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("rusqlite"));
    assert!(stdout.contains("tokio"));
    assert!(stdout.contains("DEPENDS_ON"));
}
