---
name: Problem
about: Propose a new problem type
title: "[Model] Graph3Colorability"
labels: model
assignees: ''
---

## Motivation

GRAPH 3-COLORABILITY (P9) from Garey & Johnson, Chapter 3, Section 3.3, p.76. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, Chapter 3, Section 3.3, p.76

**Mathematical definition:**

INSTANCE: Graph G=(V,E).
QUESTION: Is G 3-colorable, that is, does there exist a function f: V → {1,2,3} such that f(u) ≠ f(v) whenever {u,v} ∈ E?

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

INSTANCE: Graph G=(V,E).
QUESTION: Is G 3-colorable, that is, does there exist a function f: V → {1,2,3} such that f(u) ≠ f(v) whenever {u,v} ∈ E?

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
