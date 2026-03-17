---
name: Problem
about: Propose a new problem type
title: "[Model] OptimumCommunicationSpanningTree"
labels: model
assignees: ''
---

## Motivation

OPTIMUM COMMUNICATION SPANNING TREE (P83) from Garey & Johnson, A2 ND7. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND7

**Mathematical definition:**

INSTANCE: Complete graph G = (V,E), weight w(e) ∈ Z0+ for each e ∈ E, requirement r({u,v}) ∈ Z0+ for each pair {u,v} of vertices from V, bound B ∈ Z0+.
QUESTION: Is there a spanning tree T for G such that, if W({u,v}) denotes the sum of the weights of the edges on the path joining u and v in T, then
∑u,v ∈ V [W({u,v})·r({u,v})] ≤ B ?

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

INSTANCE: Complete graph G = (V,E), weight w(e) ∈ Z0+ for each e ∈ E, requirement r({u,v}) ∈ Z0+ for each pair {u,v} of vertices from V, bound B ∈ Z0+.
QUESTION: Is there a spanning tree T for G such that, if W({u,v}) denotes the sum of the weights of the edges on the path joining u and v in T, then

∑u,v ∈ V [W({u,v})·r({u,v})] ≤ B ?

Reference: [Johnson, Lenstra, and Rinnooy Kan, 1978]. Transformation from X3C.
Comment: Remains NP-complete even if all requirements are equal. Can be solved in polynomial time if all edge weights are equal [Hu, 1974].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
