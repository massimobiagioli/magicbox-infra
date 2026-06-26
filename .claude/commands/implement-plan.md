---
description: Implement a numbered plan from plan/ end-to-end, then record its ADR.
argument-hint: <plan-number>
---

Implement plan **$ARGUMENTS** from `plan/`.

## Steps

1. **Read** `plan/$ARGUMENTS-*.md` and every doc it references
   (CONSTITUTION, STACK, PRINCIPLES, PROJECT-STRUCTURE in `docs/`).
2. **Execute the steps in order**, using the matching skill for each:
   - CLI command ↔ playbook → `.agents/skills/add-infra-command`
   - Ansible playbook/role → `.agents/skills/write-playbook`
   - secrets / inventory / `*.example` → `.agents/skills/manage-secrets`
3. **Verify** the plan's Acceptance checklist:
   `cargo fmt` · `cargo clippy -- -D warnings` · `cargo test`.
   Every box must pass — stop and report if one cannot; never fake completion.
4. **Write the ADR** via `.agents/skills/write-adr` → `adr/$ARGUMENTS-<slug>.md`.
   Mandatory: the plan is not done until its ADR exists.
5. **Report** what changed, the checklist result, and the ADR path.

## Rules

- Stay within the plan's scope; surface extra work, don't silently add it.
- English only; secrets are never committed, logged, or printed.

## Usage example

```
/implement-plan 001
```
