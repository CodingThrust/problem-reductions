---
name: Problem
about: Propose a new problem type
title: "[Model] 3DimensionalMatching(3dm)"
labels: model
assignees: ''
---

## Motivation

3-DIMENSIONAL MATCHING (3DM) (P128) from Garey & Johnson, A3 SP1. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP1

**Mathematical definition:**

INSTANCE: Set M ⊆ W×X×Y, where W, X, and Y are disjoint sets having the same number q of elements.
QUESTION: Does M contain a matching, i.e., a subset M' ⊆ M such that |M'| = q and no two elements of M' agree in any coordinate?

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

INSTANCE: Set M ⊆ W×X×Y, where W, X, and Y are disjoint sets having the same number q of elements.
QUESTION: Does M contain a matching, i.e., a subset M' ⊆ M such that |M'| = q and no two elements of M' agree in any coordinate?
Reference: [Karp, 1972]. Transformation from 3SAT (see Section 3.1.2).
Comment: Remains NP-complete if M is "pairwise consistent," i.e., if for all elements a, b, c, whenever there exist elements w, z, and y such that (a,b,w) ∈ M, (a,x,c) ∈ M, and (y,b,c) ∈ M, then (a,b,c) ∈ M (this follows from the proof of Theorem 3.1.2). Also remains NP-complete if no element occurs in more than three triples, but is solvable in polynomial time if no element occurs in more than two triples [Garey and Johnson, ——]. The related 2-DIMENSIONAL MATCHING problem (where M ⊆ W×X) is also solvable in polynomial time (e.g., see [Lawler, 1976a]).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
