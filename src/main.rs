//! `magicbox` — the authoritative CLI for the Magicbox infrastructure.
//!
//! Rust decides *what* to do (parsing, validation, configuration) and Ansible
//! does it (state changes over SSH). With no arguments the tool prints help and
//! exits successfully; it is never destructive by default.

mod cli;
mod commands;
mod config;
mod engine;
mod error;

use clap::Parser;

use crate::cli::Cli;

fn main() -> anyhow::Result<()> {
    Cli::parse().run()
}
