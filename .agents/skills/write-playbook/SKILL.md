---
name: write-playbook
description: Write or edit Ansible playbooks and roles for the Magicbox (Alpine Linux + Docker + Portainer). Use when authoring the execution layer invoked by a CLI command.
---

# Write Playbook

Ansible is the only layer allowed to change the Magicbox. Target is Alpine
Linux; connection is SSH; everything must be idempotent.

## Conventions

- File: `ansible/playbooks/<name>.yml`, `kebab-case`, matching the CLI command.
- Target host group `magicbox` from `ansible/inventory/hosts.yml`.
- Reusable logic → `ansible/roles/`; never copy tasks between playbooks.
- Variables `snake_case` in `inventory/group_vars/all/vars.yml`; secrets in the
  encrypted `vault.yml` alongside it (see `manage-secrets`).
- Every task is **idempotent** — rely on module state, not shell scripts. Prefer
  modules over `command`/`shell`; if shell is unavoidable, guard with
  `creates`/`changed_when`/`when`.

## Alpine specifics

- Packages: `community.general.apk` (e.g. OS/lib updates with `upgrade: true`).
- Docker: `community.docker` collection for containers, stacks, registry login.
- Portainer: manage stacks/services via its API or `community.docker`.

## Checklist

- Idempotent (second run reports no changes).
- No secrets in plaintext or logs; use `no_log: true` on sensitive tasks.
- Uses `become` only where required (least privilege).
- Tagged/named tasks with clear, English names.
