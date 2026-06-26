//! Integration tests for the `magicbox` binary.
//!
//! These exercise the user-facing contract via the compiled binary: a bare
//! invocation prints help and exits 0; `--help` and the `healthcheck`
//! subcommand are wired up; unknown commands fail.

use std::process::Command;

/// Path to the binary built by Cargo for integration tests.
fn magicbox() -> Command {
    Command::new(env!("CARGO_BIN_EXE_magicbox"))
}

#[test]
fn no_args_prints_help_and_exits_zero() {
    let output = magicbox().output().expect("binary runs");
    assert!(output.status.success(), "bare invocation must exit 0");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Usage") && stdout.contains("healthcheck") && stdout.contains("os-update"),
        "help should list usage and the available commands, got:\n{stdout}"
    );
}

#[test]
fn help_flag_lists_commands() {
    let output = magicbox().arg("--help").output().expect("binary runs");
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("healthcheck"));
    assert!(stdout.contains("os-update"));
}

#[test]
fn unknown_command_fails() {
    let output = magicbox()
        .arg("definitely-not-a-command")
        .output()
        .expect("binary runs");
    assert!(!output.status.success(), "unknown command must not exit 0");
}
