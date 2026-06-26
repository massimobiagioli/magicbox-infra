# AGENTS.md

Operating guide for AI agents working on **magicbox-infra**, a Rust CLI that
controls the infrastructure of the Magicbox (a personal Alpine Linux server on
GMKTEK N150 / 16 GB RAM). The CLI delegates all state changes to Ansible.

## Golden Rules

1. **English only** — all code, comments, docs, commit messages, and output.
2. **Docs are law.** Read and obey `docs/` before changing anything.
3. **Decisions in Rust, state changes in Ansible.** Never SSH-mutate from Rust.
4. **No-arg run prints help.** Never make the CLI destructive by default.
5. **Idempotent** operations; **secrets never** committed, logged, or printed.
6. Before "done": `cargo fmt`, `cargo clippy -- -D warnings`, `cargo test`.
7. **After implementing a plan** from `plan/`, record a concise ADR in `adr/`
   using the [write-adr](./.agents/skills/write-adr/SKILL.md) skill.

## Documentation (read first)

- [docs/CONSTITUTION.md](./docs/CONSTITUTION.md) — mission, scope, core laws,
  governance. The supreme reference.
- [docs/STACK.md](./docs/STACK.md) — Rust (CLI) + Ansible (engine) and the
  boundary between them.
- [docs/PRINCIPLES.md](./docs/PRINCIPLES.md) — Clean Code, Rustacean approach,
  Security-first.
- [docs/PROJECT-STRUCTURE.md](./docs/PROJECT-STRUCTURE.md) — directory layout,
  placement rules, mandatory secret example files, README requirements.

## Skills

Located in `.agents/skills/`. Use the one that matches the task.

- [add-infra-command](./.agents/skills/add-infra-command/SKILL.md) — add a new
  CLI command wired to an Ansible playbook (OS/lib updates, Portainer stacks,
  services).
- [write-playbook](./.agents/skills/write-playbook/SKILL.md) — author/edit
  Ansible playbooks and roles for the Alpine + Docker + Portainer Magicbox.
- [manage-secrets](./.agents/skills/manage-secrets/SKILL.md) — ansible-vault,
  inventory, and the `*.example` template rule for any credential or key.
- [write-adr](./.agents/skills/write-adr/SKILL.md) — record a concise ADR in
  `adr/` after implementing a plan.

> When a new skill is added under `.agents/skills/`, reference it here.

## Commands

Custom slash commands in `.claude/commands/`.

- [/implement-plan](./.claude/commands/implement-plan.md) — implement a numbered
  plan from `plan/` end-to-end, then write its ADR. Usage: `/implement-plan 001`.

> When a new command is added under `.claude/commands/`, reference it here.

## ADR Registry

Record every ADR here immediately after writing it (rule enforced by the
`write-adr` skill). Future implementations must read this table before designing
a new approach — decisions here are binding unless explicitly superseded.

| # | Title | Status | Date |
|---|-------|--------|------|
| 001 | [Bootstrap](adr/001-bootstrap.md) | Accepted | 2026-06-25 |
| 002 | [Alpine OS Update](adr/002-alpine-update.md) | Accepted | 2026-06-25 |
| 003 | [group_vars Loading Layout](adr/003-group-vars-layout.md) | Accepted | 2026-06-25 |
| 004 | [Check Portainer Status](adr/004-check-portainer-status.md) | Accepted | 2026-06-26 |

> When a new ADR is created, append a row to this table.
