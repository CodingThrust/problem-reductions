---
name: Problem
about: Propose a new problem type
title: "[Model] MaxCut"
labels: model
assignees: ''
---

## Motivation

MAX CUT (P92) from Garey & Johnson, A2 ND16. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND16

**Mathematical definition:**

INSTANCE: Graph G = (V,E), weight w(e) ∈ Z+ for each e ∈ E, positive integer K.
QUESTION: Is there a partition of V into disjoint sets V1 and V2 such that the sum of the weights of the edges from E that have one endpoint in V1 and one endpoint in V2 is at least K?

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

INSTANCE: Graph G = (V,E), weight w(e) ∈ Z+ for each e ∈ E, positive integer K.
QUESTION: Is there a partition of V into disjoint sets V1 and V2 such that the sum of the weights of the edges from E that have one endpoint in V1 and one endpoint in V2 is at least K?

Reference: [Karp, 1972]. Transformation from MAXIMUM 2-SATISFIABILITY.
Comment: Remains NP-complete if w(e) = 1 for all e ∈ E (the SIMPLE MAX CUT problem) [Garey, Johnson, and Stockmeyer, 1976], and if, in addition, no vertex has degree exceeding 3 [Yannakakis, 1978b]. Can be solved in polynomial time if G is planar [Hadlock, 1975], [Orlova and Dorfman, 1972].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
