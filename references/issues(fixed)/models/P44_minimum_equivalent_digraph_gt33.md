---
name: Problem
about: Propose a new problem type
title: "[Model] MinimumEquivalentDigraph"
labels: model
assignees: ''
---

## Motivation

MINIMUM EQUIVALENT DIGRAPH (P44) from Garey & Johnson, A1.2 GT33. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT33

**Mathematical definition:**

INSTANCE: Directed graph G = (V, A), positive integer K ≤ |A|.
QUESTION: Is there a subset A' ⊆ A with |A'| ≤ K such that, for every ordered pair of vertices u, v ∈ V, the graph G' = (V, A') contains a directed path from u to v if and only if G does?

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

INSTANCE: Directed graph G = (V, A), positive integer K ≤ |A|.
QUESTION: Is there a subset A' ⊆ A with |A'| ≤ K such that, for every ordered pair of vertices u, v ∈ V, the graph G' = (V, A') contains a directed path from u to v if and only if G does?

Reference: [Sahni, 1974]. Transformation from DIRECTED HAMILTONIAN CIRCUIT for strongly connected graphs (see Chapter 3).

Comment: Corresponding problem in which A' ⊆ V × V instead of A' ⊆ A (called TRANSITIVE REDUCTION) can be solved in polynomial time, e.g., see [Aho, Garey, and Ullman, 1972].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
