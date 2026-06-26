# ADR 003 — group_vars Loading Layout

- **Status:** Accepted
- **Date:** 2026-06-25
- **Plan:** [plan/003-group-vars-layout.md](../plan/003-group-vars-layout.md)

## Context
`ansible/group_vars/all/` was adjacent to neither the inventory nor the
playbooks, so Ansible never auto-loaded it. Default filters hid the gap, but
the real (already-created) `vault.yml` secrets were silently not loading.

## Decision
- Amended `PROJECT-STRUCTURE.md` first (docs are law), then relocated
  `group_vars/all/{vars,vault}.yml` to `ansible/inventory/group_vars/all/`,
  adjacent to the inventory where Ansible auto-loads it.
- Updated `.gitignore`, README, `manage-secrets`/`write-playbook` skills, and
  the vault example to the new path.

## Consequences
- Group/host vars and vault secrets now load on every run; no Rust/engine change.
- The inventory path the CLI passes is unchanged; existing real `vault.yml` was
  moved (perms preserved) and stays gitignored.
