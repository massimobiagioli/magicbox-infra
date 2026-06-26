//! `healthcheck` — verify the Magicbox is reachable and healthy.
//!
//! Decision/validation only: it checks configuration, then delegates the
//! read-only probing to `ansible/playbooks/healthcheck.yml`.

use anyhow::Context;

use crate::config::Config;
use crate::engine;

/// Playbook backing this command (shares the command's base name).
const PLAYBOOK: &str = "healthcheck";

/// Runs the healthcheck playbook and reports a clear pass/fail to the user.
pub fn run(config: &Config) -> anyhow::Result<()> {
    config
        .validate()
        .context("configuration is incomplete; see README setup")?;

    println!("Checking the Magicbox is reachable and healthy...");
    match engine::run_playbook(config, PLAYBOOK) {
        Ok(()) => {
            println!("Magicbox is healthy.");
            Ok(())
        }
        Err(err) => Err(err).context("Magicbox healthcheck failed"),
    }
}
