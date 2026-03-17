---
name: Problem
about: Propose a new problem type
title: "[Model] ShortestWeightConstrainedPath"
labels: model
assignees: ''
---

## Motivation

SHORTEST WEIGHT-CONSTRAINED PATH (P106) from Garey & Johnson, A2 ND30. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND30

**Mathematical definition:**

INSTANCE: Graph G = (V,E), length l(e) ∈ Z^+, and weight w(e) ∈ Z^+ for each e ∈ E, specified vertices s,t ∈ V, positive integers K,W.
QUESTION: Is there a simple path in G from s to t with total weight W or less and total length K or less?

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

INSTANCE: Graph G = (V,E), length l(e) ∈ Z^+, and weight w(e) ∈ Z^+ for each e ∈ E, specified vertices s,t ∈ V, positive integers K,W.
QUESTION: Is there a simple path in G from s to t with total weight W or less and total length K or less?
Reference: [Megiddo, 1977]. Transformation from PARTITION.
Comment: Also NP-complete for directed graphs. Both problems are solvable in polynomial time if all weights are equal or all lengths are equal.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
