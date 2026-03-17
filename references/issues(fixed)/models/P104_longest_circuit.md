---
name: Problem
about: Propose a new problem type
title: "[Model] LongestCircuit"
labels: model
assignees: ''
---

## Motivation

LONGEST CIRCUIT (P104) from Garey & Johnson, A2 ND28. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND28

**Mathematical definition:**

INSTANCE: Graph G = (V,E), length l(e) ∈ Z^+ for each e ∈ E, positive integer K.
QUESTION: Is there a simple circuit in G of length K or more, i.e., whose edge lengths sum to at least K?

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

INSTANCE: Graph G = (V,E), length l(e) ∈ Z^+ for each e ∈ E, positive integer K.
QUESTION: Is there a simple circuit in G of length K or more, i.e., whose edge lengths sum to at least K?
Reference: Transformation from HAMILTONIAN CIRCUIT.
Comment: Remains NP-complete if l(e) = 1 for all e ∈ E, as does the corresponding problem for directed circuits in directed graphs. The directed problem with all l(e) = 1 can be solved in polynomial time if G is a "tournament" [Morrow and Goodman, 1976]. The analogous directed and undirected problems, which ask for a simple circuit of length K or less, can be solved in polynomial time (e.g., see [Itai and Rodeh, 1977b]), but are NP-complete if negative lengths are allowed.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
