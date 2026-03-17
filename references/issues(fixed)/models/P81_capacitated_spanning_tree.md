---
name: Problem
about: Propose a new problem type
title: "[Model] CapacitatedSpanningTree"
labels: model
assignees: ''
---

## Motivation

CAPACITATED SPANNING TREE (P81) from Garey & Johnson, A2 ND5. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND5

**Mathematical definition:**

INSTANCE: Graph G = (V,E), specified vertex v0 ∈ V, capacity c(e) ∈ Z0+ and length l(e) ∈ Z0+ for each e ∈ E, requirement r(v) ∈ Z0+ for each v ∈ V−{v0}, and a bound B ∈ Z0+.
QUESTION: Is there a spanning tree T for G such that the sum of the lengths of the edges in T does not exceed B and such that for each edge e in T, if U(e) is the set of vertices whose path to v0 in T contains e, then ∑u ∈ U(e) r(u) ≤ c(e)?

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

INSTANCE: Graph G = (V,E), specified vertex v0 ∈ V, capacity c(e) ∈ Z0+ and length l(e) ∈ Z0+ for each e ∈ E, requirement r(v) ∈ Z0+ for each v ∈ V−{v0}, and a bound B ∈ Z0+.
QUESTION: Is there a spanning tree T for G such that the sum of the lengths of the edges in T does not exceed B and such that for each edge e in T, if U(e) is the set of vertices whose path to v0 in T contains e, then ∑u ∈ U(e) r(u) ≤ c(e)?

Reference: [Papadimitriou, 1976c]. Transformation from 3SAT.
Comment: NP-complete in the strong sense, even if all requirements are 1 and all capacities are equal to 3. Solvable in polynomial time by weighted matching techniques if all requirements are 1 and all capacities 2. Can also be solved in polynomial time (by minimum cost network flow algorithms, e.g., see [Edmonds and Karp, 1972]) if all capacities are 1 and all requirements are either 0 or 1, but remains NP-complete if all capacities are 2, all requirements 0 or 1, and all edge lengths 0 or 1 [Even and Johnson, 1977].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
