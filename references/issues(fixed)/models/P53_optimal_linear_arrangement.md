---
name: Problem
about: Propose a new problem type
title: "[Model] OptimalLinearArrangement"
labels: model
assignees: ''
---

## Motivation

OPTIMAL LINEAR ARRANGEMENT (P53) from Garey & Johnson, A1.3 GT42. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.3 GT42

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integer K.
QUESTION: Is there a one-to-one function f: V → {1,2,...,|V|} such that ∑_{{u,v} ∈ E} |f(u)−f(v)| ≤ K?

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
QUESTION: Is there a one-to-one function f: V → {1,2,...,|V|} such that ∑_{{u,v} ∈ E} |f(u)−f(v)| ≤ K?

Reference: [Garey, Johnson, and Stockmeyer, 1976]. Transformation from SIMPLE MAX CUT.
Comment: Remains NP-complete if G is bipartite [Even and Shiloach, 1975]. Solvable in polynomial time if G is a tree [Shiloach, 1976], [Gol'dberg and Klipker, 1976].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
