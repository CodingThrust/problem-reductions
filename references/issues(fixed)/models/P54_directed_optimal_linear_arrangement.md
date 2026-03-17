---
name: Problem
about: Propose a new problem type
title: "[Model] DirectedOptimalLinearArrangement"
labels: model
assignees: ''
---

## Motivation

DIRECTED OPTIMAL LINEAR ARRANGEMENT (P54) from Garey & Johnson, A1.3 GT43. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.3 GT43

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A), positive integer K.
QUESTION: Is there a one-to-one function f: V → {1,2,...,|V|} such that f(u) < f(v) whenever (u,v) ∈ A and such that ∑_{{u,v} ∈ A} (f(v)−f(u)) ≤ K?

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

INSTANCE: Directed graph G = (V,A), positive integer K.
QUESTION: Is there a one-to-one function f: V → {1,2,...,|V|} such that f(u) < f(v) whenever (u,v) ∈ A and such that ∑_{{u,v} ∈ A} (f(v)−f(u)) ≤ K?

Reference: [Even and Shiloach, 1975]. Transformation from OPTIMAL LINEAR ARRANGEMENT.
Comment: Solvable in polynomial time if G is a tree, even if each edge has a given integer weight and the cost function is a weighted sum [Adolphson and Hu, 1973].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
