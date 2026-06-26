# ADR 001 — Bootstrap

- **Status:** Accepted
- **Date:** 2026-06-25
- **Plan:** [plan/001-bootstrap.md](../plan/001-bootstrap.md)

## Context
The project needed a first end-to-end vertical slice proving the Rust-decides /
Ansible-executes boundary works, runnable as `magicbox healthcheck`.

## Decision
- Rust CLI (`clap` derive) with `config` → `engine` → `commands/healthcheck`;
  no-arg run prints help (exit 0); `engine` only shells out to `ansible-playbook`.
- Idempotent read-only `healthcheck.yml` (ping, Alpine assert, uptime/load,
  Docker daemon via the `docker info` CLI — no Python SDK on the target —,
  optional Portainer probe).
- Secrets/inventory kept as committed `*.example` templates; real files gitignored.

## Consequences
- Running the live healthcheck requires Ansible + the `community.docker`
  collection and a populated inventory/vault (documented in README).
- `MAGICBOX_ROOT` resolves paths; later commands follow the same one-command =
  one-module + one-playbook pattern.
