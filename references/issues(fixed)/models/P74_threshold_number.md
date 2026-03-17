---
name: Problem
about: Propose a new problem type
title: "[Model] ThresholdNumber"
labels: model
assignees: ''
---

## Motivation

THRESHOLD NUMBER (P74) from Garey & Johnson, A1.5 GT63. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.5 GT63

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integer K ≤ |E|.
QUESTION: Is there a partition of E into disjoint sets E_1,E_2,...,E_K such that each of the graphs G_i = (V,E_i), 1 ≤ i ≤ K, is a "threshold graph"?

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

INSTANCE: Graph G = (V,E), positive integer K ≤ |E|.
QUESTION: Is there a partition of E into disjoint sets E_1,E_2,...,E_K such that each of the graphs G_i = (V,E_i), 1 ≤ i ≤ K, is a "threshold graph"?

Reference: [Chvátal and Hammer, 1975]. Transformation from INDEPENDENT SET restricted to triangle free graphs.
Comment: Solvable in polynomial time for K = 1.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
