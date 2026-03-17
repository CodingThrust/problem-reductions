---
name: Problem
about: Propose a new problem type
title: "[Model] K^thShortestPath"
labels: model
assignees: ''
---

## Motivation

K^th SHORTEST PATH (P107) from Garey & Johnson, A2 ND31. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND31

**Mathematical definition:**

INSTANCE: Graph G = (V,E), length l(e) ∈ Z^+ for each e ∈ E, specified vertices s,t ∈ V, positive integers B and K.
QUESTION: Are there K or more distinct simple paths from s to t in G, each having total length B or less?

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

INSTANCE: Graph G = (V,E), length l(e) ∈ Z^+ for each e ∈ E, specified vertices s,t ∈ V, positive integers B and K.
QUESTION: Are there K or more distinct simple paths from s to t in G, each having total length B or less?
Reference: [Johnson and Kashdan, 1976]. Turing reduction from HAMILTONIAN PATH.
Comment: Not known to be in NP. Corresponding K^th shortest circuit problem is also NP-hard. Both remain NP-hard if l(e) = 1 for all e ∈ E, as do the corresponding problems for directed graphs. However, all versions can be solved in pseudo-polynomial time (polynomial in |V|, K, and log B) and hence in polynomial time for any fixed value of K. The corresponding enumeration problems are #P-complete.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
