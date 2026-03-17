---
name: Problem
about: Propose a new problem type
title: "[Model] DirectedHamiltonianCircuit"
labels: model
assignees: ''
---

## Motivation

DIRECTED HAMILTONIAN CIRCUIT (P49) from Garey & Johnson, A1.3 GT38. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.3 GT38

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A).
QUESTION: Does G contain a directed Hamiltonian circuit?

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

INSTANCE: Directed graph G = (V,A).
QUESTION: Does G contain a directed Hamiltonian circuit?

Reference: [Karp, 1972]. Transformation from VERTEX COVER (see Chapter 3).
Comment: Remains NP-complete if G is planar and has no vertex involved in more than three arcs [Plesnik, 1978]. Solvable in polynomial time if no in-degree (no out-degree) exceeds 1, if G is a tournament [Morrow and Goodman, 1976], or if G is an edge digraph (e.g., see [Liu, 1968]).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
