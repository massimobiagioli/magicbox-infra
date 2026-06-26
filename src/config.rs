//! Resolves where the Ansible engine and its inputs live on disk.
//!
//! The CLI makes *decisions* here — which inventory, which playbook directory,
//! whether a vault password file is present — before handing control to the
//! engine. No state changes happen in this module.

use std::path::{Path, PathBuf};

use crate::error::InfraError;

/// Environment variable that overrides the project root used to locate the
/// `ansible/` and `secrets/` directories. Defaults to the current directory.
const ROOT_ENV: &str = "MAGICBOX_ROOT";

/// Filesystem layout the engine needs to invoke a playbook.
#[derive(Debug, Clone)]
pub struct Config {
    /// Inventory file describing the `magicbox` host group.
    inventory: PathBuf,
    /// Directory containing the playbooks.
    playbooks_dir: PathBuf,
    /// Vault password file, if one is present on disk.
    vault_password_file: Option<PathBuf>,
}

impl Config {
    /// Builds a config rooted at `MAGICBOX_ROOT` (or the current directory).
    pub fn discover() -> Self {
        let root = std::env::var_os(ROOT_ENV)
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));
        Self::from_root(root)
    }

    /// Builds a config rooted at `root`. Paths are computed, not validated;
    /// call [`Config::validate`] before driving the engine.
    pub fn from_root(root: impl AsRef<Path>) -> Self {
        let root = root.as_ref();
        let vault = root.join("secrets/.vault_pass");
        Self {
            inventory: root.join("ansible/inventory/hosts.yml"),
            playbooks_dir: root.join("ansible/playbooks"),
            vault_password_file: vault.exists().then_some(vault),
        }
    }

    /// Resolves the path of a named playbook (without the `.yml` extension).
    pub fn playbook(&self, name: &str) -> PathBuf {
        self.playbooks_dir.join(format!("{name}.yml"))
    }

    pub fn inventory(&self) -> &Path {
        &self.inventory
    }

    pub fn vault_password_file(&self) -> Option<&Path> {
        self.vault_password_file.as_deref()
    }

    /// Fails early if the inventory required to reach the Magicbox is missing.
    ///
    /// The vault password file is intentionally optional: a playbook that uses
    /// no secrets can run without it.
    pub fn validate(&self) -> Result<(), InfraError> {
        if !self.inventory.exists() {
            return Err(InfraError::MissingFile(self.inventory.clone()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playbook_path_is_derived_from_root() {
        let config = Config::from_root("/srv/magicbox");
        assert_eq!(
            config.playbook("healthcheck"),
            PathBuf::from("/srv/magicbox/ansible/playbooks/healthcheck.yml")
        );
    }

    #[test]
    fn inventory_path_is_derived_from_root() {
        let config = Config::from_root("/srv/magicbox");
        assert_eq!(
            config.inventory(),
            Path::new("/srv/magicbox/ansible/inventory/hosts.yml")
        );
    }

    #[test]
    fn validate_reports_missing_inventory() {
        let config = Config::from_root("/nonexistent-magicbox-root");
        assert!(matches!(config.validate(), Err(InfraError::MissingFile(_))));
    }
}
