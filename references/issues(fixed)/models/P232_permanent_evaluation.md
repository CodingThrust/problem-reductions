---
name: Problem
about: Propose a new problem type
title: "[Model] PermanentEvaluation"
labels: model
assignees: ''
---

## Motivation

PERMANENT EVALUATION (P232) from Garey & Johnson, A7 AN13. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN13

**Mathematical definition:**

INSTANCE: An n×n matrix M of 0's and 1's, and a positive integer K ≤ n!.
QUESTION: Is the value of the permanent of M equal to K?

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

INSTANCE: An n×n matrix M of 0's and 1's, and a positive integer K ≤ n!.
QUESTION: Is the value of the permanent of M equal to K?

Reference: [Valiant, 1977a]. Transformation from 3SAT.
Comment: The problem is NP-hard but not known to be in NP, as is the case for the variants in which we ask whether the value of the permanent is "K or less" or "K or more." The problem of computing the value of the permanent of M is #P-complete.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
