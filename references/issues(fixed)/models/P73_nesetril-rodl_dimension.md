---
name: Problem
about: Propose a new problem type
title: "[Model] NesetrilRödlDimension"
labels: model
assignees: ''
---

## Motivation

NESETRIL-RÖDL DIMENSION (P73) from Garey & Johnson, A1.5 GT62. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.5 GT62

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integer K ≤ |E|.
QUESTION: Is there a one-to-one function f: V → {(a_1,a_2,...,a_K): 1 ≤ a_i ≤ |V| for 1 ≤ i ≤ K} such that, for all u,v ∈ V, {u,v} ∈ E if and only if f(u) and f(v) disagree in all K components?

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

INSTANCE: Graph G = (V,E), positive integer K ≤ |E|.
QUESTION: Is there a one-to-one function f: V → {(a_1,a_2,...,a_K): 1 ≤ a_i ≤ |V| for 1 ≤ i ≤ K} such that, for all u,v ∈ V, {u,v} ∈ E if and only if f(u) and f(v) disagree in all K components?

Reference: [Nesetril and Pultr, 1977]. Transformation from GRAPH 3-COLORABILITY. The definition appears in [Nesetril and Rödl, 1977].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
