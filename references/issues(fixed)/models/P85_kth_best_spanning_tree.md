---
name: Problem
about: Propose a new problem type
title: "[Model] KthBestSpanningTree"
labels: model
assignees: ''
---

## Motivation

Kth BEST SPANNING TREE (P85) from Garey & Johnson, A2 ND9. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND9

**Mathematical definition:**

INSTANCE: Graph G = (V,E), weight w(e) ∈ Z0+ for each e ∈ E, positive integers K and B.
QUESTION: Are there K distinct spanning trees for G, each having total weight B or less?

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

INSTANCE: Graph G = (V,E), weight w(e) ∈ Z0+ for each e ∈ E, positive integers K and B.
QUESTION: Are there K distinct spanning trees for G, each having total weight B or less?

Reference: [Johnson and Kashdan, 1976]. Turing reduction from HAMILTONIAN PATH.
Comment: Not known to be in NP. Can be solved in pseudo-polynomial time (polynomial in |V|, K, log B, max {log w(e): e ∈ E}) [Lawler, 1972], and hence in polynomial time for any fixed value of K. The corresponding enumeration problem is #P-complete. However, the unweighted case of the enumeration problem is solvable in polynomial time (e.g., see [Harary and Palmer, 1973]).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
