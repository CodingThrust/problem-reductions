---
name: Problem
about: Propose a new problem type
title: "[Model] BottleneckTravelingSalesman"
labels: model
assignees: ''
---

## Motivation

BOTTLENECK TRAVELING SALESMAN (P100) from Garey & Johnson, A2 ND24. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND24

**Mathematical definition:**

INSTANCE: Set C of m cities, distance d(ci,cj) ∈ Z+ for each pair of cities ci,cj ∈ C, positive integer B.
QUESTION: Is there a tour of C whose longest edge is no longer than B, i.e., a permutation <cπ(1),cπ(2),...,cπ(m)> of C such that d(cπ(i),cπ(i+1)) ≤ B for 1 ≤ i < m and such that d(cπ(m),cπ(1)) ≤ B?

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

INSTANCE: Set C of m cities, distance d(ci,cj) ∈ Z+ for each pair of cities ci,cj ∈ C, positive integer B.
QUESTION: Is there a tour of C whose longest edge is no longer than B, i.e., a permutation <cπ(1),cπ(2),...,cπ(m)> of C such that d(cπ(i),cπ(i+1)) ≤ B for 1 ≤ i < m and such that d(cπ(m),cπ(1)) ≤ B?

Reference: Transformation from HAMILTONIAN CIRCUIT.
Comment: Remains NP-complete even if d(ci,cj) ∈ {1,2} for all ci,cj ∈ C. An important special case that is solvable in polynomial time can be found in [Gilmore and Gomory, 1964].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
