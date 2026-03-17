---
name: Problem
about: Propose a new problem type
title: "[Model] WeightedDiameter"
labels: model
assignees: ''
---

## Motivation

WEIGHTED DIAMETER (P76) from Garey & Johnson, A1.5 GT65. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.5 GT65

**Mathematical definition:**

INSTANCE: Graph G = (V,E), collection C of |E| not necessarily distinct non-negative integers, positive integer K.
QUESTION: Is there a one-to-one function f: E → C such that, if f(e) is taken as the length of edge e, then G has diameter K or less, i.e., every pair of points u,v ∈ V is joined by a path in G of length K or less.

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

INSTANCE: Graph G = (V,E), collection C of |E| not necessarily distinct non-negative integers, positive integer K.
QUESTION: Is there a one-to-one function f: E → C such that, if f(e) is taken as the length of edge e, then G has diameter K or less, i.e., every pair of points u,v ∈ V is joined by a path in G of length K or less.

Reference: [Perl and Zaks, 1978]. Transformation from 3-PARTITION.
Comment: NP-complete in the strong sense, even if G is a tree. The variant in which "diameter" is replaced by "radius" has the same complexity. If C consists entirely of 0's and 1's, then both the diameter and radius versions are solvable in polynomial time for trees, but are NP-complete for general graphs, even if K is fixed at 2 (diameter) or 1 (radius). The variant in which we ask for an assignment yielding diameter K or greater is NP-complete in the strong sense for general graphs, is solvable in polynomial time for trees in the diameter case, and is NP-complete for trees in the radius case.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
