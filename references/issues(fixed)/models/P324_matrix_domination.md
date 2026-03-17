---
name: Problem
about: Propose a new problem type
title: "[Model] MatrixDomination"
labels: model
assignees: ''
---

## Motivation

MATRIX DOMINATION (P324) from Garey & Johnson, A12 MS12. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS12

**Mathematical definition:**

INSTANCE: An n×n matrix M with entries from {0,1}, and a positive integer K.
QUESTION: Is there a set of K or fewer non-zero entries in M that dominate all others, i.e., s subset C ⊆ {1,2,...,n}×{1,2,...,n} with |C| ≤ K such that Mij = 1 for all (i,j) ∈ C and such that, whenever Mij = 1, there exists an (i',j') ∈ C for which either i = i' or j = j'?

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

INSTANCE: An n×n matrix M with entries from {0,1}, and a positive integer K.
QUESTION: Is there a set of K or fewer non-zero entries in M that dominate all others, i.e., s subset C ⊆ {1,2,...,n}×{1,2,...,n} with |C| ≤ K such that Mij = 1 for all (i,j) ∈ C and such that, whenever Mij = 1, there exists an (i',j') ∈ C for which either i = i' or j = j'?
Reference: [Yannakakis and Gavril, 1978]. Transformation from MINIMUM MAXIMAL MATCHING.
Comment: Remains NP-complete even if M is upper triangular.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
