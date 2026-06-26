# ADR 004 — Check Portainer Status

- **Status:** Accepted
- **Date:** 2026-06-26
- **Plan:** [plan/004-check-portainer-status.md](../plan/004-check-portainer-status.md)

## Context

The healthcheck had an optional, non-failing Portainer probe gated on an empty
`portainer_healthcheck_url`. No dedicated command existed to verify Portainer
reachability independently.

## Decision

- Set `portainer_healthcheck_url` in `group_vars/all/vars.yml` using
  `{{ ansible_host }}` so the hostname is never hardcoded.
- Added `ansible/playbooks/portainer-status.yml`: a dedicated, failing probe of
  `https://{{ ansible_host }}:9443/api/status` (HTTP 200/401/403 accepted).
- Added `magicbox portainer-status` CLI command following the `healthcheck`
  pattern (`src/commands/portainer_status.rs`).

## Consequences

- `make healthcheck` now also runs the soft Portainer probe (side effect of
  populating `portainer_healthcheck_url`).
- Portainer host/port are configured in one place (`vars.yml`); changing
  `ansible_host` in inventory propagates automatically.
