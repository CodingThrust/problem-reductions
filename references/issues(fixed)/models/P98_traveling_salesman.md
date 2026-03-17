---
name: Problem
about: Propose a new problem type
title: "[Model] TravelingSalesman"
labels: model
assignees: ''
---

## Motivation

TRAVELING SALESMAN (P98) from Garey & Johnson, A2 ND22. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND22

**Mathematical definition:**

INSTANCE: Set C of m cities, distance d(ci,cj) ∈ Z+ for each pair of cities ci,cj ∈ C, positive integer B.
QUESTION: Is there a tour of C having length B or less, i.e., a permutation <cπ(1),cπ(2),...,cπ(m)> of C such that
(∑i=1 to m−1 d(cπ(i),cπ(i+1))) + d(cπ(m),cπ(1)) ≤ B ?

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
QUESTION: Is there a tour of C having length B or less, i.e., a permutation <cπ(1),cπ(2),...,cπ(m)> of C such that

(∑i=1 to m−1 d(cπ(i),cπ(i+1))) + d(cπ(m),cπ(1)) ≤ B ?

Reference: Transformation from HAMILTONIAN CIRCUIT.
Comment: Remains NP-complete even if d(ci,cj) ∈ {1,2} for all ci,cj ∈ C. Special cases that can be solved in polynomial time are discussed in [Gilmore and Gomory, 1964], [Garfinkel, 1977], and [Syslo, 1973]. The variant in which we ask for a tour with "mean arrival time" of B or less is also NP-complete [Sahni and Gonzalez, 1976].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
