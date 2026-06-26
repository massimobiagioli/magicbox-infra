//! `portainer-status` — verify the Portainer instance is reachable.
//!
//! Decision/validation only: it checks configuration, then delegates the
//! read-only probe to `ansible/playbooks/portainer-status.yml`.

use anyhow::Context;

use crate::config::Config;
use crate::engine;

const PLAYBOOK: &str = "portainer-status";

pub fn run(config: &Config) -> anyhow::Result<()> {
    config
        .validate()
        .context("configuration is incomplete; see README setup")?;

    println!("Checking Portainer status...");
    match engine::run_playbook(config, PLAYBOOK) {
        Ok(()) => {
            println!("Portainer is reachable.");
            Ok(())
        }
        Err(err) => Err(err).context("Portainer status check failed"),
    }
}
