---
name: Problem
about: Propose a new problem type
title: "[Model] Bandwidth"
labels: model
assignees: ''
---

## Motivation

BANDWIDTH (P51) from Garey & Johnson, A1.3 GT40. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.3 GT40

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
QUESTION: Is there a linear ordering of V with bandwidth K or less, i.e., a one-to-one function f: V → {1,2,...,|V|} such that, for all {u,v} ∈ E, |f(u)−f(v)| ≤ K?

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

INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
QUESTION: Is there a linear ordering of V with bandwidth K or less, i.e., a one-to-one function f: V → {1,2,...,|V|} such that, for all {u,v} ∈ E, |f(u)−f(v)| ≤ K?

Reference: [Papadimitriou, 1976a]. Transformation from 3-PARTITION.
Comment: Remains NP-complete for trees with no vertex degree exceeding 3 [Garey, Graham, Johnson, and Knuth, 1978]. This problem corresponds to that of minimizing the "bandwidth" of a symmetric matrix by simultaneous row and column permutations.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
