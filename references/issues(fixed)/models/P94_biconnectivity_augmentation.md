---
name: Problem
about: Propose a new problem type
title: "[Model] BiconnectivityAugmentation"
labels: model
assignees: ''
---

## Motivation

BICONNECTIVITY AUGMENTATION (P94) from Garey & Johnson, A2 ND18. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND18

**Mathematical definition:**

INSTANCE: Graph G = (V,E), weight w({u,v}) ∈ Z+ for each unordered pair {u,v} of vertices from V, positive integer B.
QUESTION: Is there a set E' of unordered pairs of vertices from V such that ∑e ∈ E' w(e) ≤ B and such that the graph G' = (V,E∪E') is biconnected, i.e., cannot be disconnected by removing a single vertex?

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

INSTANCE: Graph G = (V,E), weight w({u,v}) ∈ Z+ for each unordered pair {u,v} of vertices from V, positive integer B.
QUESTION: Is there a set E' of unordered pairs of vertices from V such that ∑e ∈ E' w(e) ≤ B and such that the graph G' = (V,E∪E') is biconnected, i.e., cannot be disconnected by removing a single vertex?

Reference: [Eswaran and Tarjan, 1976]. Transformation from HAMILTONIAN CIRCUIT.
Comment: The related problem in which G' must be bridge connected, i.e., cannot be disconnected by removing a single edge, is also NP-complete. Both problems remain NP-complete if all weights are either 1 or 2 and E is empty. Both can be solved in polynomial time if all weights are equal.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
