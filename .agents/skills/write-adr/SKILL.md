---
name: write-adr
description: Record an Architecture Decision Record after implementing a plan. Use right after a plan in plan/ is implemented. ADRs are intentionally tiny to keep context light.
---

# Write ADR

After a plan is implemented, capture the decision in `adr/`. Keep it minimal —
record the decision, not a narrative.

## Steps

1. File: `adr/NNN-<slug>.md`, where `NNN` matches the plan number (e.g. plan
   `001-bootstrap` → `adr/001-bootstrap.md`).
2. Use the template below. Stay within ~15 lines. No filler.

## Template

```markdown
# ADR NNN — <Title>

- **Status:** Accepted
- **Date:** YYYY-MM-DD
- **Plan:** [plan/NNN-<slug>.md](../plan/NNN-<slug>.md)

## Context
<1–2 sentences: the problem.>

## Decision
<1–3 bullets: what was decided/done.>

## Consequences
<1–2 bullets: trade-offs / follow-ups.>
```

## Rules

- One ADR per implemented plan; never edit a past ADR — supersede it with a new
  one referencing the old.
- Concise over complete; link instead of restating docs.
