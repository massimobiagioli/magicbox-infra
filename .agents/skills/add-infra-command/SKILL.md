---
name: add-infra-command
description: Add a new CLI command to magicbox-infra that maps to an Ansible playbook (OS/lib updates, Portainer stacks, services). Use whenever a new infrastructure operation must be exposed by the CLI.
---

# Add Infra Command

A command = a Rust subcommand (decision/validation) + an Ansible playbook
(state change). Same base name in both. Decisions in Rust, mutations in Ansible.

## Steps

1. **Define the command** in `src/cli.rs` (clap): name in `kebab-case`, args,
   help text. Verify no-arg run still prints help.
2. **Module** `src/commands/<name>.rs`: validate input, build playbook params,
   call `engine.rs`. Add to `src/commands/mod.rs`. No SSH/state changes here.
3. **Playbook** `ansible/playbooks/<name>.yml` — see the `write-playbook` skill.
   Extract shared logic into `ansible/roles/`.
4. **Secrets**: if it needs new secrets, follow the `manage-secrets` skill and
   update the matching `*.example`.
5. **Test** the Rust parsing/param-building in `tests/`.
6. **Verify**: `cargo fmt`, `cargo clippy -- -D warnings`, `cargo test`.
7. **Docs**: if behavior changes, update `README.md` usage and relevant `docs/`.

## Rules

- Idempotent end-to-end (re-runnable safely).
- Never print or log secrets.
- One operation per command; reuse roles, don't duplicate.
