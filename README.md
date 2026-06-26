# magicbox-infra

The authoritative command-line utility for the **Magicbox** — a personal Alpine
Linux server (GMKTEK N150, 16 GB RAM) running Docker and Portainer. The CLI is
written in Rust and delegates every state change to Ansible over SSH.

See [`docs/`](./docs) for the project law: the
[Constitution](./docs/CONSTITUTION.md), [Stack](./docs/STACK.md),
[Principles](./docs/PRINCIPLES.md), and
[Project Structure](./docs/PROJECT-STRUCTURE.md).

## Prerequisites

On your machine (the Ansible controller):

- **Rust toolchain** (stable) — `cargo`, `rustc`.
- **Ansible** with the `community.docker` and `community.general` collections:
  ```bash
  ansible-galaxy collection install community.docker community.general
  ```
- **SSH access** to the Magicbox (key-based), reachable on your LAN.

On the Magicbox (the managed node):

- **Python 3** — Ansible runs its modules through a Python interpreter on the
  target, and Alpine does not ship one by default. Install it once **as root**
  on the Magicbox — Alpine has no `sudo`, so log in as root or `su -` first:
  ```sh
  apk add --no-cache python3
  ```
  It lands at `/usr/bin/python3`, matching `ansible_python_interpreter` in the
  inventory. Without it, even `healthcheck` fails at fact gathering with
  `/bin/sh: python3: not found`.

- **Avahi** — makes the Magicbox discoverable on the local network as
  `magicbox.local` via mDNS (Multicast DNS, also known as Bonjour on macOS).
  Without it, `magicbox.local` only resolves from the Mac side (where Bonjour
  is built in); the Magicbox itself and any other Linux host on the network
  cannot resolve the name.

  Install and enable it once **as root**:
  ```sh
  apk add avahi avahi-tools dbus
  rc-update add dbus boot
  rc-update add avahi-daemon boot
  rc-service dbus start
  rc-service avahi-daemon start
  ```

  How it works: Avahi implements the mDNS/DNS-SD protocol (RFC 6762). It
  multicasts the hostname over UDP port 5353 so that any device on the same
  subnet can resolve `<hostname>.local` without a DNS server. The `dbus`
  daemon is a required dependency for Avahi on Alpine. The `avahi-tools`
  package provides `avahi-resolve` and `avahi-browse` for troubleshooting.

  Verify the name resolves from another machine on the network:
  ```sh
  avahi-resolve -n magicbox.local   # from another Linux host with avahi
  dns-sd -G v4 magicbox.local       # from macOS
  ```

## Setup

### 1. SSH key

Ansible connects to the Magicbox over SSH using key authentication (no
passwords). If you don't already have a dedicated key, create one and install
its **public** half on the Magicbox.

```bash
# Generate a dedicated key pair (skip if you reuse an existing key).
ssh-keygen -t ed25519 -C "magicbox-infra" -f ~/.ssh/magicbox

# Copy the PUBLIC key to the Magicbox (uses your current password access once).
ssh-copy-id -i ~/.ssh/magicbox.pub <ssh_user>@192.168.x.x

# Verify a passwordless login works.
ssh -i ~/.ssh/magicbox <ssh_user>@192.168.x.x 'echo connected'
```

Notes:
- Keep the **private** key (`~/.ssh/magicbox`) on your machine only — never
  commit it. `chmod 600 ~/.ssh/magicbox` if its permissions are too open.
- The first connection records the host key; `ansible.cfg` keeps
  `host_key_checking = True`, so accept it once via the `ssh` command above.
- The `<ssh_user>` should be able to reach the Docker daemon (e.g. in the
  `docker` group), or the healthcheck's Docker step will fail.

### 2. Configuration files

Copy each `*.example` template to its real, gitignored counterpart and fill it
in. Point `ansible_ssh_private_key_file` at the private key from step 1. Real
secrets are **never committed**.

```bash
# Inventory: where the Magicbox lives and how to reach it.
cp ansible/inventory/hosts.example.yml ansible/inventory/hosts.yml
# ...then set ansible_host, ansible_user, and
# ansible_ssh_private_key_file: ~/.ssh/magicbox

# Vault: real secrets, then encrypt them with ansible-vault.
cp secrets/vars.example.yml ansible/inventory/group_vars/all/vault.yml
ansible-vault encrypt ansible/inventory/group_vars/all/vault.yml

# Vault password file used to decrypt the vault at runtime.
cp secrets/.vault_pass.example secrets/.vault_pass
# ...then edit secrets/.vault_pass and set the real password.
```

The CLI looks for these files relative to the current directory by default. Set
`MAGICBOX_ROOT` to run it from elsewhere.

## Build

```bash
cargo build --release
# binary: target/release/magicbox
```

## Usage

Running with no arguments prints help and exits 0 — the tool is never
destructive by default:

```bash
magicbox
```

### Commands

| Command       | Description                                     | Playbook                        |
| ------------- | ----------------------------------------------- | ------------------------------- |
| `healthcheck` | Check the Magicbox is reachable and healthy.    | `ansible/playbooks/healthcheck.yml` |
| `os-update`   | Update and upgrade Alpine packages.             | `ansible/playbooks/os-update.yml`   |

```bash
magicbox healthcheck
```

`healthcheck` is read-only: it verifies SSH connectivity, confirms the host runs
Alpine Linux, reports uptime/load, checks the Docker daemon is up, and
optionally probes a Portainer endpoint (set `portainer_healthcheck_url` in
`ansible/inventory/group_vars/all/vars.yml` to enable it). A clear pass/fail and a
meaningful exit code are returned.

```bash
magicbox os-update
```

`os-update` **changes system state**: it refreshes the `apk` index and upgrades
all installed packages (idempotent — a second run upgrades nothing). Only the
upgrade step escalates to root.

Escalation defaults live in `ansible/inventory/group_vars/all/vars.yml`
(`ansible_become_method: doas`, `os_update_become: true`). Alpine ships **no
`sudo`**, so privilege escalation uses `doas`. For a non-interactive run the SSH
user needs a **NOPASS `doas`** rule — set this up once on the Magicbox, as root:

```sh
apk add doas                                       # if not already installed
echo 'permit nopass <ssh_user> as root' > /etc/doas.d/30-<ssh_user>.conf
doas -C /etc/doas.d/30-<ssh_user>.conf && echo "config OK"
```

Put the rule in `/etc/doas.d/`, **not** in `/etc/doas.conf`. Files in
`/etc/doas.d/*.conf` are read *after* `doas.conf`, and `doas` applies the **last
matching rule**. Alpine ships `/etc/doas.d/20-wheel.conf` (`permit persist
:wheel`); if your SSH user is in the `wheel` group, that rule overrides a NOPASS
line placed in `doas.conf` and escalation still demands a password (you'll see
`doas: Authentication required`, which Ansible surfaces as the misleading
`Module result deserialization failed: No start of json char found`). The `30-`
prefix sorts after `20-wheel.conf`, so the NOPASS rule wins.

Verify it works non-interactively, **as the SSH user**:

```sh
doas -n true && echo "NOPASS works"
```

If instead you connect **as root**, no escalation is needed — set
`os_update_become: false` (in `group_vars/all/vars.yml` or per-host in your
inventory).

It also reports whether a reboot is recommended (a newer kernel was installed)
but never reboots on its own.

Further capabilities (library updates, Portainer init, service management) are
added as new commands in later plans.

## Security note

Never commit real secrets. Only `*.example` templates belong in version control;
`ansible/inventory/hosts.yml`, `ansible/inventory/group_vars/all/vault.yml`, and
`secrets/.vault_pass` are gitignored. Secrets are never logged or printed.

## Development

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```
