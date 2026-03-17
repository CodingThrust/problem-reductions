---
name: Problem
about: Propose a new problem type
title: "[Model] NonTautology"
labels: model
assignees: ''
---

## Motivation

NON-TAUTOLOGY (P260) from Garey & Johnson, A9 LO8. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A9 LO8

**Mathematical definition:**

INSTANCE: Boolean expression E over a set U of variables, using the connectives "¬" (not), "∨" (or), "∧" (and), and "→" (implies).
QUESTION: Is E not a tautology, i.e., is there a truth assignment for U that makes E false?

## Variables

- **Count:** (TBD)
- **Per-variable domain:** (TBD)
- **Meaning:** (TBD)

## Schema (data type)

**Type name:** (TBD)
**Variants:** (TBD)

| Field | Type | Description |
|-------|------|-------------|
| (TBD) | (TBD) | (TBD) |

## Complexity

- **Best known exact algorithm:** (TBD)

## Extra Remark

**Full book text:**

INSTANCE: Boolean expression E over a set U of variables, using the connectives "¬" (not), "∨" (or), "∧" (and), and "→" (implies).
QUESTION: Is E not a tautology, i.e., is there a truth assignment for U that makes E false?
Reference: [Cook, 1971a]. Transformation from SATISFIABILITY.
Comment: Remains NP-complete even if E is in "disjunctive normal form" with at most 3 literals per disjunct.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
