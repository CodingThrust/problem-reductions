---
name: Problem
about: Propose a new problem type
title: "[Model] SatisfiabilityOfBooleanExpressions"
labels: model
assignees: ''
---

## Motivation

SATISFIABILITY OF BOOLEAN EXPRESSIONS (P259) from Garey & Johnson, A9 LO7. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A9 LO7

**Mathematical definition:**

INSTANCE: Variable set U, a subset B of the set of 16 possible binary Boolean connectives, and a well-formed Boolean expression E over U and B.
QUESTION: Is there a truth assignment for U that satisfies E?

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

INSTANCE: Variable set U, a subset B of the set of 16 possible binary Boolean connectives, and a well-formed Boolean expression E over U and B.
QUESTION: Is there a truth assignment for U that satisfies E?
Reference: [Cook, 1971a]. Generic transformation.
Comment: Remains NP-complete if B is restricted to {∧,∨,→,¬}, or any other truth-functionally complete set of connectives. Also NP-complete for any truth-functionally incomplete set of connectives containing {↦}, {↤}, {≢,∨}, or {≢,∧} as a subset [Lewis, 1978]. Problem is solvable in polynomial time for any truth-functionally incomplete set of connectives not containing one of these four sets as a subset.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
