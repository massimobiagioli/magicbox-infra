//! `os-update` — update and upgrade Alpine packages on the Magicbox.
//!
//! Decision/validation only: it checks configuration, then delegates the actual
//! state change (`apk update` + `apk upgrade`) to
//! `ansible/playbooks/os-update.yml`. The playbook streams which packages were
//! upgraded, or reports the system was already up to date.

use anyhow::Context;

use crate::config::Config;
use crate::engine;

/// Playbook backing this command (shares the command's base name).
const PLAYBOOK: &str = "os-update";

/// Runs the OS-update playbook and reports a clear outcome to the user.
pub fn run(config: &Config) -> anyhow::Result<()> {
    config
        .validate()
        .context("configuration is incomplete; see README setup")?;

    println!("Updating and upgrading Alpine packages on the Magicbox...");
    match engine::run_playbook(config, PLAYBOOK) {
        Ok(()) => {
            println!("Alpine OS update completed.");
            Ok(())
        }
        Err(err) => Err(err).context("Alpine OS update failed"),
    }
}
