---
name: Problem
about: Propose a new problem type
title: "[Model] HamiltonianCircuit"
labels: model
assignees: ''
---

## Motivation

HAMILTONIAN CIRCUIT (P48) from Garey & Johnson, A1.3 GT37. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.3 GT37

**Mathematical definition:**

INSTANCE: Graph G = (V,E).
QUESTION: Does G contain a Hamiltonian circuit?

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

INSTANCE: Graph G = (V,E).
QUESTION: Does G contain a Hamiltonian circuit?

Reference: [Karp, 1972]. Transformation from VERTEX COVER (see Chapter 3).
Comment: Remains NP-complete (1) if G is planar, cubic, 3-connected, and has no face with fewer than 5 edges [Garey, Johnson, and Tarjan, 1976a], (2) if G is bipartite [Krishnamoorthy, 1975], (3) if G is the square of a graph [Chvátal, 1976], and (4) if a Hamiltonian path for G is given as part of the instance [Papadimitriou and Stieglitz, 1976]. Solvable in polynomial time if G has no vertex with degree exceeding 2 or if G is an edge graph (e.g., see [Liu, 1968]). The cube of a non-trivial connected graph always has a Hamiltonian circuit [Karaganis, 1968].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
