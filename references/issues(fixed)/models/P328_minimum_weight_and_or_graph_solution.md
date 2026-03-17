---
name: Problem
about: Propose a new problem type
title: "[Model] MinimumWeightAnd/orGraphSolution"
labels: model
assignees: ''
---

## Motivation

MINIMUM WEIGHT AND/OR GRAPH SOLUTION (P328) from Garey & Johnson, A12 MS16. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS16

**Mathematical definition:**

INSTANCE: Directed acyclic graph G = (V,A) with a single vertex s ∈ V having in-degree 0, assignment f(v) ∈ {and,or} for each v ∈ V having nonzero out-degree, weight w(a) ∈ Z+ for each a ∈ A, and a positive integer K.
QUESTION: Is there a subgraph G' = (V',A') of G such that s ∈ V', such that if v ∈ V' and f(v) = and then all arcs leaving v in A belong to A', such that if v ∈ V' and f(v) = or then at least one of the arcs leaving v in A belongs to A', and such that the sum of the weights of the arcs in A' does not exceed K?

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

INSTANCE: Directed acyclic graph G = (V,A) with a single vertex s ∈ V having in-degree 0, assignment f(v) ∈ {and,or} for each v ∈ V having nonzero out-degree, weight w(a) ∈ Z+ for each a ∈ A, and a positive integer K.
QUESTION: Is there a subgraph G' = (V',A') of G such that s ∈ V', such that if v ∈ V' and f(v) = and then all arcs leaving v in A belong to A', such that if v ∈ V' and f(v) = or then at least one of the arcs leaving v in A belongs to A', and such that the sum of the weights of the arcs in A' does not exceed K?
Reference: [Sahni, 1974]. Transformation from X3C.
Comment: Remains NP-complete even if w(a) = 1 for all a ∈ A [Garey and Johnson, ——]. The general problem is solvable in polynomial time for rooted directed trees by dynamic programming.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
