---
name: Problem
about: Propose a new problem type
title: "[Model] TotalUnimodularity"
labels: model
assignees: ''
---

## Motivation

TOTAL UNIMODULARITY (P341) from Garey & Johnson, A13 OPEN10. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A13 OPEN10

**Mathematical definition:**

INSTANCE: An m × n matrix M with entries from the set {-1, 0, 1}.
QUESTION: Is M not totally unimodular, i.e., is there a square submatrix of M whose determinant is not in the set {-1, 0, 1}?

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

[OPEN10] TOTAL UNIMODULARITY
INSTANCE: An m × n matrix M with entries from the set {-1, 0, 1}.
QUESTION: Is M not totally unimodular, i.e., is there a square submatrix of M whose determinant is not in the set {-1, 0, 1}?
Comment: The problem remains open even if all entries in M are from {0, 1}. The significance of totally unimodular matrices for integer programming is discussed, for example, in [Lawler, 1976] and [Garfinkel and Nemhauser, 1972].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
