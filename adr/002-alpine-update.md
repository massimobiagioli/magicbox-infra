# ADR 002 — Alpine OS Update

- **Status:** Accepted
- **Date:** 2026-06-25
- **Plan:** [plan/002-alpine-update.md](../plan/002-alpine-update.md)

## Context
The Magicbox needs a first-class, repeatable way to keep Alpine patched, and the
first **state-changing** CLI command to prove the mutation path end to end.

## Decision
- Added the `os-update` command (`src/commands/os_update.rs`) reusing the
  existing `engine` unchanged — decisions in Rust, mutation in Ansible.
- `os-update.yml` runs `community.general.apk` (`update_cache` + `upgrade`),
  idempotently, escalating via `become` (`ansible_become_method`, default `doas`).
- Advisory, read-only reboot hint (newer kernel installed) — never reboots.

## Consequences
- The SSH user must be able to escalate to root (`doas`/`sudo`) on the Magicbox.
- `community.general` collection is required on the controller.
- Reboots, library/runtime updates, and kernel pinning remain for later plans.
