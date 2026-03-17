---
name: Problem
about: Propose a new problem type
title: "[Model] HamiltonianCompletion"
labels: model
assignees: ''
---

## Motivation

HAMILTONIAN COMPLETION (P45) from Garey & Johnson, A1.2 GT34. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT34

**Mathematical definition:**

INSTANCE: Graph G = (V,E), non-negative integer K ≤ |V|.
QUESTION: Is there a superset E' containing E such that |E'-E| ≤ K and the graph G' = (V,E') has a Hamiltonian circuit?

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

INSTANCE: Graph G = (V,E), non-negative integer K ≤ |V|.
QUESTION: Is there a superset E' containing E such that |E'-E| ≤ K and the graph G' = (V,E') has a Hamiltonian circuit?
Reference: Transformation from HAMILTONIAN CIRCUIT.
Comment: Remains NP-complete for any fixed K ≥ 0. Corresponding "completion" versions of HAMILTONIAN PATH, DIRECTED HAMILTONIAN PATH, and DIRECTED HAMILTONIAN CIRCUIT are also NP-complete. HAMILTONIAN COMPLETION and HAMILTONIAN PATH COMPLETION can be solved in polynomial time if G is a tree [Boesch, Chen, and McHugh, 1974].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
