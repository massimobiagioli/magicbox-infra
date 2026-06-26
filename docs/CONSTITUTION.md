# Magicbox Infra — Constitution

> The supreme reference document for this project. Every decision, line of code,
> and playbook MUST comply with this Constitution. When other documents conflict
> with it, this document prevails.

## 1. Mission

`magicbox-infra` is a command-line utility that fully controls the
infrastructure of the **Magicbox**, a personal server. The CLI is the single,
authoritative entry point for every infrastructure operation. Nothing on the
Magicbox should be changed by hand once this tool exists — if an operation is
worth doing, it is worth encoding as a command.

## 2. The Magicbox

| Property      | Value                                  |
| ------------- | -------------------------------------- |
| Role          | Personal server                        |
| Hardware      | GMKTEK N150, 16 GB RAM                  |
| OS            | Alpine Linux                           |
| Container host| Docker                                 |
| Orchestration | Portainer                              |
| Registry      | Private container registry             |
| Access        | SSH                                    |

### 2.1 Manual Baseline (pre-existing)

The following was provisioned **by hand** and is assumed to already exist. The
CLI builds on top of it and MUST NOT need to recreate it:

- Operating system installed (Alpine Linux)
- Docker installed and running
- SSH access configured
- Portainer installed
- Private registry configured

Everything beyond this baseline is the responsibility of the CLI.

## 3. Scope

The CLI MUST be able to perform any infrastructure operation, including but not
limited to:

- **OS updates** — keep Alpine Linux patched and current.
- **Library / package updates** — system packages and dependencies.
- **Dev stack initialization** — bootstrap the Portainer development stack.
- **Service management** — add, update, and remove services in Portainer.

New capabilities are added as new commands; the scope grows by extension, never
by working around the CLI.

## 4. Core Laws

1. **The CLI is the source of truth.** All infrastructure changes flow through
   it. Manual changes to the Magicbox are forbidden except for the documented
   baseline.
2. **Help by default.** Running the CLI with no arguments MUST print the help.
   The tool is never destructive without an explicit command.
3. **Idempotency.** Every operation can be run repeatedly with the same end
   state. Re-running a command never corrupts the system.
4. **Declarative over imperative.** Desired state is described, not scripted
   step-by-step. Ansible is the execution engine (see [STACK.md](./STACK.md)).
5. **Security-first.** Secrets are never committed, never logged, never printed.
   See [PRINCIPLES.md](./PRINCIPLES.md).
6. **Reproducibility.** A fresh Magicbox (after the manual baseline) can be
   brought to full working state using only this CLI.
7. **Least privilege.** Operations request only the access they need.

## 5. Governance

- This Constitution, together with [STACK.md](./STACK.md),
  [PRINCIPLES.md](./PRINCIPLES.md) and
  [PROJECT-STRUCTURE.md](./PROJECT-STRUCTURE.md), forms the law of the project.
- Amendments are deliberate: change the document first, then the code follows.
- Any code or playbook that violates this Constitution is a defect, regardless
  of whether it "works".

## 6. Language

All code, comments, documentation, commit messages, and command output are
written in **English**.
