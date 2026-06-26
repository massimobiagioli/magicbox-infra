---
name: manage-secrets
description: Handle secrets for magicbox-infra (ansible-vault, inventory, example templates). Use whenever adding, reading, or rotating any credential, key, or sensitive variable.
---

# Manage Secrets

Security-first: real secrets are never committed, logged, or printed.

## Rules

- Real secret files are **gitignored**: `ansible/inventory/group_vars/all/vault.yml`,
  `ansible/inventory/hosts.yml`, `secrets/.vault_pass`, any `*.local.*`.
- **Every** secret file has a committed `*.example` template with placeholder
  values only. Adding a secret = update its example in the same change.
- Encrypt real secrets with `ansible-vault`; reference them as `snake_case`
  vars in playbooks.

## Common operations

```bash
# Encrypt / edit the vault
ansible-vault encrypt ansible/inventory/group_vars/all/vault.yml
ansible-vault edit    ansible/inventory/group_vars/all/vault.yml

# Vault password file (gitignored)
cp secrets/.vault_pass.example secrets/.vault_pass   # then fill it

# Bootstrap real files from templates
cp ansible/inventory/hosts.example.yml ansible/inventory/hosts.yml
cp secrets/vars.example.yml ansible/inventory/group_vars/all/vault.yml
```

## In playbooks

- Sensitive tasks use `no_log: true`.
- Never echo a secret to stdout or into error messages.
- Confirm the file is gitignored before committing anything nearby.
