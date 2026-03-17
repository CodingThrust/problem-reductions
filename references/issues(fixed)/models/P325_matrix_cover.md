---
name: Problem
about: Propose a new problem type
title: "[Model] MatrixCover"
labels: model
assignees: ''
---

## Motivation

MATRIX COVER (P325) from Garey & Johnson, A12 MS13. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS13

**Mathematical definition:**

INSTANCE: An n×n matrix A = (aij) with nonnegative integer entries, and an integer K.
QUESTION: Is there a function f: {1,2,...,n}→{−1,+1} such that
∑1≤i,j≤n aij·f(i)·f(j) ≥ K ?

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

INSTANCE: An n×n matrix A = (aij) with nonnegative integer entries, and an integer K.
QUESTION: Is there a function f: {1,2,...,n}→{−1,+1} such that
∑1≤i,j≤n aij·f(i)·f(j) ≥ K ?
Reference: [Garey and Johnson, ——]. Transformation from MAX CUT.
Comment: NP-complete in the strong sense and remains so if A is required to be positive definite.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
