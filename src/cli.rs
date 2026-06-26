//! Command-line interface definition (clap).
//!
//! Parsing and help live here; running a command with no subcommand prints
//! help and exits 0 (see [`Cli::run`]). Each subcommand maps to a module in
//! [`crate::commands`].

use clap::{Parser, Subcommand};

use crate::commands;
use crate::config::Config;

/// `magicbox` — the single entry point for Magicbox infrastructure operations.
#[derive(Debug, Parser)]
#[command(
    name = "magicbox",
    about = "Control the Magicbox infrastructure via Ansible.",
    long_about = "magicbox is the authoritative CLI for the Magicbox server. \
Every operation is a subcommand backed by an idempotent Ansible playbook. \
Run without arguments to see the available commands.",
    version,
    // Make a bare `magicbox` invocation print help instead of erroring.
    subcommand_required = false,
    arg_required_else_help = false
)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The set of infrastructure operations the CLI exposes.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Check the Magicbox is reachable and healthy.
    Healthcheck,
    /// Update and upgrade Alpine packages on the Magicbox.
    OsUpdate,
    /// Check that the Portainer instance is reachable.
    PortainerStatus,
}

impl Cli {
    /// Dispatches the parsed command. With no subcommand, prints help (exit 0).
    pub fn run(self) -> anyhow::Result<()> {
        match self.command {
            Some(Commands::Healthcheck) => {
                let config = Config::discover();
                commands::healthcheck::run(&config)
            }
            Some(Commands::OsUpdate) => {
                let config = Config::discover();
                commands::os_update::run(&config)
            }
            Some(Commands::PortainerStatus) => {
                let config = Config::discover();
                commands::portainer_status::run(&config)
            }
            None => {
                Self::print_help();
                Ok(())
            }
        }
    }

    /// Renders the same help text clap produces for `--help`.
    fn print_help() {
        use clap::CommandFactory;
        // `print_help` writes to stdout; ignore the unlikely write error.
        let _ = Self::command().print_help();
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_definition_is_valid() {
        // Catches clap configuration errors (duplicate args, bad help, etc.).
        Cli::command().debug_assert();
    }

    #[test]
    fn no_args_parses_to_no_subcommand() {
        let cli = Cli::try_parse_from(["magicbox"]).expect("bare invocation parses");
        assert!(cli.command.is_none());
    }

    #[test]
    fn healthcheck_subcommand_parses() {
        let cli = Cli::try_parse_from(["magicbox", "healthcheck"]).expect("healthcheck parses");
        assert!(matches!(cli.command, Some(Commands::Healthcheck)));
    }

    #[test]
    fn os_update_subcommand_parses_in_kebab_case() {
        let cli = Cli::try_parse_from(["magicbox", "os-update"]).expect("os-update parses");
        assert!(matches!(cli.command, Some(Commands::OsUpdate)));
    }

    #[test]
    fn unknown_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["magicbox", "bogus"]).is_err());
    }
}
