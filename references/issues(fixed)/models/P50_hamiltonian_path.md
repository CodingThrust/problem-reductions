---
name: Problem
about: Propose a new problem type
title: "[Model] HamiltonianPath"
labels: model
assignees: ''
---

## Motivation

HAMILTONIAN PATH (P50) from Garey & Johnson, A1.3 GT39. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.3 GT39

**Mathematical definition:**

INSTANCE: Graph G = (V,E).
QUESTION: Does G contain a Hamiltonian path?

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
QUESTION: Does G contain a Hamiltonian path?

Reference: Transformation from VERTEX COVER (see Chapter 3).
Comment: Remains NP-complete under restrictions (1) and (2) for HAMILTONIAN CIRCUIT and is polynomially solvable under the same restrictions as HC. Corresponding DIRECTED HAMILTONIAN PATH problem is also NP-complete, and the comments for DIRECTED HC apply to it as well. The variants in which either the starting point or the ending point or both are specified in the instance are also NP-complete. DIRECTED HAMILTONIAN PATH can be solved in polynomial time for acyclic digraphs, e.g., see [Lawler, 1976a].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
