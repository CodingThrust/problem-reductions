---
name: Problem
about: Propose a new problem type
title: "[Model] 3MatroidIntersection"
labels: model
assignees: ''
---

## Motivation

3-MATROID INTERSECTION (P138) from Garey & Johnson, A3 SP11. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP11

**Mathematical definition:**

INSTANCE: Three matroids (E,F_1),(E,F_2),(E,F_3), positive integer K ≤ |E|. (A matroid (E,F) consists of a set E of elements and a non-empty family F of subsets of E such that (1) S ∈ F implies all subsets of S are in F and (2) if two sets S,S' ∈ F satisfy |S| = |S'|+1, then there exists an element e ∈ S − S' such that (S'∪{e}) ∈ F.)
QUESTION: Is there a subset E' ⊆ E such that |E'| = K and E' ∈ (F_1 ∩ F_2 ∩ F_3)?

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

INSTANCE: Three matroids (E,F_1),(E,F_2),(E,F_3), positive integer K ≤ |E|. (A matroid (E,F) consists of a set E of elements and a non-empty family F of subsets of E such that (1) S ∈ F implies all subsets of S are in F and (2) if two sets S,S' ∈ F satisfy |S| = |S'|+1, then there exists an element e ∈ S − S' such that (S'∪{e}) ∈ F.)
QUESTION: Is there a subset E' ⊆ E such that |E'| = K and E' ∈ (F_1 ∩ F_2 ∩ F_3)?
Reference: Transformation from 3DM.
Comment: The related 2-MATROID INTERSECTION problem can be solved in polynomial time, even if the matroids are described by giving polynomial time algorithms for recognizing their members, and even if each element e ∈ E has a weight w(e) ∈ Z^+, with the goal being to find an E' ∈ (F_1 ∩ F_2) having maximum total weight (e.g., see [Lawler, 1976a]).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
