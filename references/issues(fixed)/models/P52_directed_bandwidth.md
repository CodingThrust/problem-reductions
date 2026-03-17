---
name: Problem
about: Propose a new problem type
title: "[Model] DirectedBandwidth"
labels: model
assignees: ''
---

## Motivation

DIRECTED BANDWIDTH (P52) from Garey & Johnson, A1.3 GT41. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.3 GT41

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A), positive integer K ≤ |V|.
QUESTION: Is there a one-to-one function f: V → {1,2,...,|V|} such that, for all (u,v) ∈ A, f(u) < f(v) and (f(v)−f(u)) ≤ K?

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

INSTANCE: Directed graph G = (V,A), positive integer K ≤ |V|.
QUESTION: Is there a one-to-one function f: V → {1,2,...,|V|} such that, for all (u,v) ∈ A, f(u) < f(v) and (f(v)−f(u)) ≤ K?

Reference: [Garey, Graham, Johnson, and Knuth, 1978]. Transformation from 3-PARTITION.
Comment: Remains NP-complete for rooted directed trees with maximum in-degree 1 and maximum out-degree at most 2. This problem corresponds to that of minimizing the "bandwidth" of an upper triangular matrix by simultaneous row and column permutations.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
