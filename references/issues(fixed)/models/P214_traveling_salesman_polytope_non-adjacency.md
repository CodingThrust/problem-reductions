---
name: Problem
about: Propose a new problem type
title: "[Model] TravelingSalesmanPolytopeNonAdjacency"
labels: model
assignees: ''
---

## Motivation

TRAVELING SALESMAN POLYTOPE NON-ADJACENCY (P214) from Garey & Johnson, A6 MP8. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A6 MP8

**Mathematical definition:**

INSTANCE: Graph G = (V,E), two Hamiltonian circuits C and C' for G.
QUESTION: Do C and C' correspond to non-adjacent vertices of the "traveling salesman polytope" for G?

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

INSTANCE: Graph G = (V,E), two Hamiltonian circuits C and C' for G.
QUESTION: Do C and C' correspond to non-adjacent vertices of the "traveling salesman polytope" for G?

Reference: [Papadimitriou, 1978a]. Transformation from 3SAT.
Comment: Result also holds for the "non-symmetric" case where G is a directed graph and C and C' are directed Hamiltonian circuits. Analogous polytope non-adjacency problems for graph matching and CLIQUE can be solved in polynomial time [Chvátal, 1975].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
