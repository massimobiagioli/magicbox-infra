# Magicbox Infra — Stack

The project is intentionally built from two layers with a strict separation of
concerns.

## 1. Rust — CLI Application

Rust is the **interface and control layer**. It is the program the user runs.

Responsibilities:

- Parse arguments and subcommands; print help when invoked with no arguments.
- Validate input before anything touches the Magicbox.
- Resolve configuration, inventory, and secrets.
- Orchestrate the engine: select the right playbook, build its parameters, and
  invoke it.
- Render clear, structured output and meaningful exit codes.

Why Rust:

- Single self-contained binary — easy to ship and run.
- Strong type system catches errors at compile time.
- Fast startup and low overhead.
- Safe by default (memory safety, explicit error handling).

Suggested crates: `clap` (CLI parsing/help), `anyhow`/`thiserror` (errors),
`serde` + `serde_yaml` (config), `tokio`/`std::process` (process control).

## 2. Ansible — Engine

Ansible is the **execution layer**. It performs the actual changes on the
Magicbox.

Responsibilities:

- Define desired state declaratively (playbooks and roles).
- Connect to the Magicbox over SSH.
- Apply OS updates, package updates, Portainer stacks, and services.
- Guarantee idempotency.

Why Ansible:

- Declarative and idempotent by design.
- Agentless — only SSH is required on the Magicbox.
- Mature modules for packages, Docker, and files.
- Secrets handled via `ansible-vault`.

## 3. How the Layers Interact

```
user ──> Rust CLI ──> selects & parametrizes ──> Ansible playbook ──SSH──> Magicbox
            │                                          │
            └── validation, config, secrets            └── idempotent state changes
```

- The CLI **never** SSHes into the Magicbox directly to mutate state; it always
  delegates state changes to Ansible.
- Ansible **never** decides user-facing behavior; it only executes what the CLI
  asks for.

## 4. Boundary Rule

If a task involves **deciding what to do**, it belongs in Rust.
If a task involves **changing the Magicbox**, it belongs in Ansible.
