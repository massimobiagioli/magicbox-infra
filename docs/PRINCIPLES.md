# Magicbox Infra — Principles

These principles govern *how* the project is built. They complement the
[Constitution](./CONSTITUTION.md) and apply to both the Rust CLI and the Ansible
engine.

## 1. Clean Code

- **Clear names.** Functions, variables, commands, and playbooks say what they
  do. No abbreviations that need a glossary.
- **Small units.** Functions and tasks do one thing. If it needs "and" to
  describe it, split it.
- **No duplication (DRY).** Shared logic lives in one place — a module, a role,
  or a reusable task.
- **Readability over cleverness.** Code is read far more than it is written.
- **Comments explain *why*, not *what*.** The code already says what.
- **Fail loudly, fail early.** Validate inputs at the boundary; never proceed on
  bad state.
- **Tests where they matter.** Argument parsing, parameter building, and
  validation are covered by tests.

## 2. Rustacean Approach

- **Make illegal states unrepresentable.** Use enums and newtypes; encode
  invariants in the type system instead of checking at runtime.
- **Errors are values.** Use `Result<T, E>`; reserve `panic!` for true bugs.
  Propagate with `?`. Use `thiserror` for library errors, `anyhow` at the CLI
  boundary.
- **No `unwrap()` / `expect()` in production paths.** Allowed only in tests or
  where an invariant is provably upheld and documented.
- **Borrow, don't clone, by default.** Clone only when ownership is genuinely
  needed.
- **Embrace the type system.** Prefer iterators, pattern matching, `Option`
  combinators, and `From`/`Into` conversions.
- **Idiomatic tooling is mandatory.** `cargo fmt` and `cargo clippy` (warnings
  treated as errors) must pass before any change is considered done.
- **Modules with clear boundaries.** Public API is deliberate; internals stay
  private.

## 3. Security-First

- **Secrets never touch the repository.** No credentials, keys, tokens, or vault
  passwords in version control. Commit only `*.example` templates (see
  [PROJECT-STRUCTURE.md](./PROJECT-STRUCTURE.md)).
- **Encrypt at rest.** Real secrets live in `ansible-vault`-encrypted files.
- **Never log or print secrets.** Output and error messages are scrubbed of
  sensitive values.
- **Least privilege.** SSH and service operations request only the access they
  need. Avoid blanket root where a scoped permission works.
- **Validate all input.** Treat command arguments and config as untrusted until
  validated.
- **Safe defaults.** The tool does nothing destructive without an explicit
  command; no surprise mutations.
- **Auditable actions.** Every operation is traceable to a command and a
  playbook — no hidden side effects.
- **Keep it patched.** Updating the OS and dependencies is a first-class
  capability, not an afterthought.

## 4. Precedence

When principles tension with each other, resolve in this order:

1. Security-first
2. Correctness & idempotency (Constitution)
3. Clean Code / Rustacean idioms
