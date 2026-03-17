---
name: Problem
about: Propose a new problem type
title: "[Model] MaximumLikelihoodRanking"
labels: model
assignees: ''
---

## Motivation

MAXIMUM LIKELIHOOD RANKING (P323) from Garey & Johnson, A12 MS11. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS11

**Mathematical definition:**

INSTANCE: An n×n matrix A = (aij) with integer entries satisfying aij + aji = 0 for all i,j ∈ {1,2,...,n}, positive integer B.
QUESTION: Is there a matrix B = (bij) obtained from A by simultaneous row and column permutations such that
∑1≤i<j≤n min{bij,0} ≥ −B ?

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

INSTANCE: An n×n matrix A = (aij) with integer entries satisfying aij + aji = 0 for all i,j ∈ {1,2,...,n}, positive integer B.
QUESTION: Is there a matrix B = (bij) obtained from A by simultaneous row and column permutations such that
∑1≤i<j≤n min{bij,0} ≥ −B ?
Reference: [Rafsky, 1977]. Transformation from FEEDBACK ARC SET.
Comment: NP-complete in the strong sense.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
