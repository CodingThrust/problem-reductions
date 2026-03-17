---
name: Problem
about: Propose a new problem type
title: "[Model] EdgeEmbeddingOnAGrid"
labels: model
assignees: ''
---

## Motivation

EDGE EMBEDDING ON A GRID (P123) from Garey & Johnson, A2 ND47. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND47

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integers M,N.
QUESTION: Is there a one-to-one function f: V → {1,2,…,M}×{1,2,…,N} such that if {u,v} ∈ E, f(u) = (x_1,y_1), and f(v) = (x_2,y_2), then either x_1 = x_2 or y_1 = y_2, i.e., f(u) and f(v) are both on the same "line" of the grid?

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

INSTANCE: Graph G = (V,E), positive integers M,N.
QUESTION: Is there a one-to-one function f: V → {1,2,…,M}×{1,2,…,N} such that if {u,v} ∈ E, f(u) = (x_1,y_1), and f(v) = (x_2,y_2), then either x_1 = x_2 or y_1 = y_2, i.e., f(u) and f(v) are both on the same "line" of the grid?
Reference: [Gavril, 1977a]. Transformation from 3-PARTITION.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
