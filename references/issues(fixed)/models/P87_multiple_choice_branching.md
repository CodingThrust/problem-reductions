---
name: Problem
about: Propose a new problem type
title: "[Model] MultipleChoiceBranching"
labels: model
assignees: ''
---

## Motivation

MULTIPLE CHOICE BRANCHING (P87) from Garey & Johnson, A2 ND11. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND11

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A), a weight w(a) ∈ Z+ for each arc a ∈ A, a partition of A into disjoint sets A1,A2,...,Am, and a positive integer K.
QUESTION: Is there a subset A' ∈ A with ∑a ∈ A' w(a) ≥ K such that no two arcs in A' enter the same vertex, A' contains no cycles, and A' contains at most one arc from each of the Ai, 1 ≤ i ≤ m?

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

INSTANCE: Directed graph G = (V,A), a weight w(a) ∈ Z+ for each arc a ∈ A, a partition of A into disjoint sets A1,A2,...,Am, and a positive integer K.
QUESTION: Is there a subset A' ∈ A with ∑a ∈ A' w(a) ≥ K such that no two arcs in A' enter the same vertex, A' contains no cycles, and A' contains at most one arc from each of the Ai, 1 ≤ i ≤ m?

Reference: [Garey and Johnson, ——]. Transformation from 3SAT.
Comment: Remains NP-complete even if G is strongly connected and all weights are equal. If all Ai have |Ai| = 1, the problem becomes simply that of finding a "maximum weight branching," a 2-matroid intersection problem that can be solved in polynomial time (e.g., see [Tarjan, 1977]). (In a strongly connected graph, a maximum weight branching can be viewed as a maximum weight directed spanning tree.) Similarly, if the graph is symmetric, the problem becomes equivalent to the "multiple choice spanning tree" problem, another 2-matroid intersection problem that can be solved in polynomial time [Suurballe, 1975].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
