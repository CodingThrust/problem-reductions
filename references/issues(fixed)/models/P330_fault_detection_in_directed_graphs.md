---
name: Problem
about: Propose a new problem type
title: "[Model] FaultDetectionInDirectedGraphs"
labels: model
assignees: ''
---

## Motivation

FAULT DETECTION IN DIRECTED GRAPHS (P330) from Garey & Johnson, A12 MS18. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS18

**Mathematical definition:**

INSTANCE: Directed acyclic graph G = (V,A), with I ⊆ V denoting those vertices with in-degree 0 and O ⊆ V denoting those vertices with out-degree 0, and a positive integer K.
QUESTION: Is there a "test set" of size K or less that can detect every "single fault" in G, i.e., is there a subset T ⊆ I×O with |T| ≤ K such that, for every v ∈ V, there exists some pair (u1,u2) ∈ T such that v is on a directed path from u1 to u2 in G?

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

INSTANCE: Directed acyclic graph G = (V,A), with I ⊆ V denoting those vertices with in-degree 0 and O ⊆ V denoting those vertices with out-degree 0, and a positive integer K.
QUESTION: Is there a "test set" of size K or less that can detect every "single fault" in G, i.e., is there a subset T ⊆ I×O with |T| ≤ K such that, for every v ∈ V, there exists some pair (u1,u2) ∈ T such that v is on a directed path from u1 to u2 in G?
Reference: [Ibaraki, Kameda, and Toida, 1977]. Transformation from X3C.
Comment: Remains NP-complete even if |O| = 1. Variant in which we ask that T be sufficient for "locating" any single fault, i.e., that for every pair v,v' ∈ V there is some (u1,u2) ∈ T such that v is on a directed path from u1 to u2 but v' is on no such path, is also NP-complete for |O| = 1. Both problems can be solved in polynomial time if K ≥ |I|·|O|.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
