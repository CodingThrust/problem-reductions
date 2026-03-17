---
name: Problem
about: Propose a new problem type
title: "[Model] GraphGrundyNumbering"
labels: model
assignees: ''
---

## Motivation

GRAPH GRUNDY NUMBERING (P67) from Garey & Johnson, A1.5 GT56. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.5 GT56

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A).
QUESTION: Is there a function f: V → Z^+ such that, for each v ∈ V, f(v) is the least non-negative integer not contained in the set {f(u): u ∈ V,(v,u) ∈ A}?

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

INSTANCE: Directed graph G = (V,A).
QUESTION: Is there a function f: V → Z^+ such that, for each v ∈ V, f(v) is the least non-negative integer not contained in the set {f(u): u ∈ V,(v,u) ∈ A}?

Reference: [van Leeuwen, 1976a]. Transformation from 3SAT.
Comment: Remains NP-complete when restricted to planar graphs in which no vertex has in- or out-degree exceeding 5 [Fraenkel and Yesha, 1977].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
