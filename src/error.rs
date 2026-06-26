//! Error types for the magicbox CLI.
//!
//! Library-style errors use `thiserror`; the CLI boundary (`main`) wraps them
//! with `anyhow` for context-rich reporting.

use std::path::PathBuf;
use std::process::ExitStatus;

use thiserror::Error;

/// Errors raised while resolving configuration or driving the Ansible engine.
#[derive(Debug, Error)]
pub enum InfraError {
    /// A required file (inventory, playbook, vault password) is missing.
    #[error("required file not found: {0}")]
    MissingFile(PathBuf),

    /// The `ansible-playbook` binary could not be launched.
    #[error("failed to launch '{program}': {source}")]
    Spawn {
        program: String,
        #[source]
        source: std::io::Error,
    },

    /// The playbook ran but returned a non-success exit status.
    #[error("playbook '{playbook}' failed ({status})")]
    PlaybookFailed {
        playbook: String,
        status: ExitStatus,
    },
}
