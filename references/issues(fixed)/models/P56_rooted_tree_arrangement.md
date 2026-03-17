---
name: Problem
about: Propose a new problem type
title: "[Model] RootedTreeArrangement"
labels: model
assignees: ''
---

## Motivation

ROOTED TREE ARRANGEMENT (P56) from Garey & Johnson, A1.3 GT45. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.3 GT45

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integer K.
QUESTION: Is there a rooted tree T = (U,F), with |U| = |V|, and a one-to-one function f: V → U such that for every edge {u,v} ∈ E there is a simple path from the root that includes both f(u) and f(v) and such that if d(x,y) is the number of edges on the path from x to y in T, then ∑_{{u,v} ∈ E} d(f(u),f(v)) ≤ K?

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

INSTANCE: Graph G = (V,E), positive integer K.
QUESTION: Is there a rooted tree T = (U,F), with |U| = |V|, and a one-to-one function f: V → U such that for every edge {u,v} ∈ E there is a simple path from the root that includes both f(u) and f(v) and such that if d(x,y) is the number of edges on the path from x to y in T, then ∑_{{u,v} ∈ E} d(f(u),f(v)) ≤ K?

Reference: [Gavril, 1977a]. Transformation from OPTIMAL LINEAR ARRANGEMENT.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
