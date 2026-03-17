---
name: Problem
about: Propose a new problem type
title: "[Model] ConsecutiveOnesMatrixPartition"
labels: model
assignees: ''
---

## Motivation

CONSECUTIVE ONES MATRIX PARTITION (P163) from Garey & Johnson, A4 SR15. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR15

**Mathematical definition:**

INSTANCE: An m×n matrix A of 0's and 1's.
QUESTION: Can the rows of A be partitioned into two groups such that the resulting m1×n and m2×n matrices (m1 + m2 = m) each have the consecutive ones property?

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

INSTANCE: An m×n matrix A of 0's and 1's.
QUESTION: Can the rows of A be partitioned into two groups such that the resulting m1×n and m2×n matrices (m1 + m2 = m) each have the consecutive ones property?
Reference: [Lipsky, 1978]. Transformation from HAMILTONIAN PATH for cubic graphs.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
