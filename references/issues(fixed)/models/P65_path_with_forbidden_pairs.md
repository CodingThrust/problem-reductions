---
name: Problem
about: Propose a new problem type
title: "[Model] PathWithForbiddenPairs"
labels: model
assignees: ''
---

## Motivation

PATH WITH FORBIDDEN PAIRS (P65) from Garey & Johnson, A1.5 GT54. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.5 GT54

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A), specified vertices s,t ∈ V, collection C = {(a_1,b_1),...,(a_n,b_n)} of pairs of vertices from V.
QUESTION: Is there a directed path from s to t in G that contains at most one vertex from each pair in C?

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

INSTANCE: Directed graph G = (V,A), specified vertices s,t ∈ V, collection C = {(a_1,b_1),...,(a_n,b_n)} of pairs of vertices from V.
QUESTION: Is there a directed path from s to t in G that contains at most one vertex from each pair in C?

Reference: [Gabow, Maheshwari, and Osterweil, 1976]. Transformation from 3SAT.
Comment: Remains NP-complete even if G is acyclic with no in- or out-degree exceeding 2. Variant in which the "forbidden pairs" are arcs instead of vertices is also NP-complete under the same restrictions. Both problems remain NP-complete even if all the given pairs are required to be disjoint.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
