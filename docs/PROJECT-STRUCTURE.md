# Magicbox Infra — Project Structure

This document defines the directory layout and the rules that keep the project
consistent. It is binding: new files go where this document says they go.

## 1. Layout

```
magicbox-infra/
├── AGENTS.md                  # Entry point for AI agents (root)
├── README.md                  # Human usage instructions (root, mandatory)
├── Cargo.toml                 # Rust manifest
├── Cargo.lock
├── .gitignore
│
├── docs/                      # Project law & reference
│   ├── CONSTITUTION.md
│   ├── STACK.md
│   ├── PRINCIPLES.md
│   └── PROJECT-STRUCTURE.md
│
├── .agents/
│   └── skills/                # Agent skills (one folder per skill)
│       ├── add-infra-command/
│       ├── write-playbook/
│       └── manage-secrets/
│
├── src/                       # Rust CLI source
│   ├── main.rs                # Entry point; no args => print help
│   ├── cli.rs                 # clap definitions (commands, args, help)
│   ├── config.rs              # Config & inventory loading
│   ├── engine.rs              # Ansible invocation wrapper
│   ├── error.rs               # Error types
│   └── commands/              # One module per subcommand
│       ├── mod.rs
│       ├── os_update.rs
│       ├── libs_update.rs
│       ├── portainer_init.rs
│       └── portainer_add_service.rs
│
├── ansible/                   # The execution engine
│   ├── ansible.cfg
│   ├── inventory/
│   │   ├── hosts.example.yml  # Template (committed)
│   │   ├── hosts.yml          # Real inventory (gitignored)
│   │   └── group_vars/        # Auto-loaded: adjacent to the inventory
│   │       └── all/
│   │           ├── vars.yml   # Non-secret variables (committed)
│   │           └── vault.yml  # Encrypted secrets (gitignored)
│   ├── playbooks/             # One playbook per CLI operation
│   │   ├── os-update.yml
│   │   ├── libs-update.yml
│   │   ├── portainer-init.yml
│   │   └── portainer-add-service.yml
│   └── roles/                 # Reusable roles shared by playbooks
│
├── secrets/                   # Secret templates & local secrets
│   ├── .vault_pass.example    # Template for the vault password file
│   └── vars.example.yml       # Template listing required secret keys
│
└── tests/                     # Rust integration tests
```

## 2. Rules

### 2.1 Placement

- **One subcommand = one Rust module** in `src/commands/` **+ one playbook** in
  `ansible/playbooks/`. They share the same kebab/snake base name.
- **Reusable Ansible logic** goes into `ansible/roles/`, never copy-pasted
  between playbooks.
- **Decisions live in Rust, state changes live in Ansible** (see
  [STACK.md](./STACK.md)). No SSH-based mutation from Rust.
- **Docs are law.** Anything that changes behavior is reflected in `docs/`
  before or alongside the code.

### 2.2 Secrets — MANDATORY

- Real secrets are **never committed**. The following are always gitignored:
  `ansible/inventory/hosts.yml`, `ansible/inventory/group_vars/all/vault.yml`,
  `secrets/.vault_pass`, and any `*.local.*`.
- For **every** secret-bearing file there MUST be a committed `*.example`
  counterpart that documents the required keys with placeholder values and no
  real data. Adding a new secret means updating its example file in the same
  change.
- Encrypted secrets use `ansible-vault`. See the
  [manage-secrets](../.agents/skills/manage-secrets/) skill.

### 2.3 README

- A `README.md` at the repository root is **mandatory** and kept current. It
  contains the instructions to install, configure, and use the CLI (see §4).

### 2.4 Naming

- Rust: `snake_case` files and items; commands exposed in `kebab-case`.
- Ansible: `kebab-case` playbook files; `snake_case` variables.
- Example files: `<name>.example` or `<name>.example.<ext>`.

## 3. Required Example Files

| Real file (gitignored)             | Committed template            |
| ---------------------------------- | ----------------------------- |
| `ansible/inventory/hosts.yml`      | `ansible/inventory/hosts.example.yml` |
| `ansible/inventory/group_vars/all/vault.yml` | `secrets/vars.example.yml` |
| `secrets/.vault_pass`              | `secrets/.vault_pass.example` |

`hosts.example.yml` (illustrative):

```yaml
all:
  hosts:
    magicbox:
      ansible_host: 192.168.x.x        # Magicbox LAN address
      ansible_user: <ssh_user>
      ansible_ssh_private_key_file: ~/.ssh/<key>
```

`vars.example.yml` (illustrative — keys only, no real values):

```yaml
registry_url: "registry.example.local:5000"
registry_username: "<username>"
registry_password: "<password>"     # encrypted in the real vault.yml
portainer_admin_password: "<password>"
```

## 4. README Contents (minimum)

The root `README.md` MUST cover:

1. **What it is** — one-line description and a pointer to `docs/`.
2. **Prerequisites** — Rust toolchain, Ansible, SSH access to the Magicbox.
3. **Setup** — copy each `*.example` to its real file and fill it in; create the
   `ansible-vault` password file.
4. **Build** — `cargo build --release`.
5. **Usage** — running the binary with no arguments prints help; document each
   command (OS update, libs update, Portainer init, add service).
6. **Security note** — never commit real secrets.
