//! Thin wrapper around `ansible-playbook`.
//!
//! This is the only place the CLI shells out to the execution layer. The engine
//! never SSHes into the Magicbox itself — it builds the command line, streams
//! the playbook's output to the terminal, and maps the exit status to a
//! [`Result`]. All state changes belong to the playbook it invokes.

use std::ffi::OsString;
use std::path::Path;
use std::process::Command;

use crate::config::Config;
use crate::error::InfraError;

/// Name of the Ansible binary the engine drives.
const ANSIBLE_PLAYBOOK: &str = "ansible-playbook";

/// Runs `playbook_name` against the Magicbox inventory described by `config`.
///
/// Output is inherited (streamed live); on a non-zero exit the playbook name
/// and status are surfaced as an [`InfraError`].
pub fn run_playbook(config: &Config, playbook_name: &str) -> Result<(), InfraError> {
    let playbook = config.playbook(playbook_name);
    if !playbook.exists() {
        return Err(InfraError::MissingFile(playbook));
    }

    let status = Command::new(ANSIBLE_PLAYBOOK)
        .args(build_args(config, playbook_name))
        .status()
        .map_err(|source| InfraError::Spawn {
            program: ANSIBLE_PLAYBOOK.to_string(),
            source,
        })?;

    if status.success() {
        Ok(())
    } else {
        Err(InfraError::PlaybookFailed {
            playbook: playbook_name.to_string(),
            status,
        })
    }
}

/// Builds the argument vector for a playbook invocation, without running it.
///
/// Exposed so argument construction can be unit-tested in isolation from the
/// real `ansible-playbook` process.
pub fn build_args(config: &Config, playbook_name: &str) -> Vec<OsString> {
    let mut args: Vec<OsString> = vec![
        OsString::from("-i"),
        config.inventory().as_os_str().to_owned(),
        config.playbook(playbook_name).into_os_string(),
    ];
    if let Some(vault) = config.vault_password_file() {
        push_vault_args(&mut args, vault);
    }
    args
}

fn push_vault_args(args: &mut Vec<OsString>, vault: &Path) {
    args.push(OsString::from("--vault-password-file"));
    args.push(vault.as_os_str().to_owned());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn args_include_inventory_and_playbook() {
        let config = Config::from_root("/srv/magicbox");
        let args = build_args(&config, "healthcheck");
        assert_eq!(args[0], OsString::from("-i"));
        assert_eq!(
            args[1],
            OsString::from("/srv/magicbox/ansible/inventory/hosts.yml")
        );
        assert_eq!(
            args[2],
            OsString::from("/srv/magicbox/ansible/playbooks/healthcheck.yml")
        );
    }

    #[test]
    fn vault_args_are_omitted_when_no_vault_file_present() {
        // A throwaway root has no secrets/.vault_pass, so no vault flag.
        let config = Config::from_root("/srv/magicbox");
        let args = build_args(&config, "healthcheck");
        assert!(!args.iter().any(|a| a == "--vault-password-file"));
    }
}
