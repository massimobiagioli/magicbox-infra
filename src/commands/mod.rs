//! One module per CLI subcommand.
//!
//! Each command validates input and parametrizes the engine; none of them
//! mutate the Magicbox directly (see [`crate::engine`]).

pub mod healthcheck;
pub mod os_update;
pub mod portainer_status;
