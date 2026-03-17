---
name: Problem
about: Propose a new problem type
title: "[Model] MinimumCutLinearArrangement"
labels: model
assignees: ''
---

## Motivation

MINIMUM CUT LINEAR ARRANGEMENT (P55) from Garey & Johnson, A1.3 GT44. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.3 GT44

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integer K.
QUESTION: Is there a one-to-one function f: V → {1,2,...,|V|} such that for all i, 1 < i < |V|,
|{{u,v} ∈ E: f(u) ≤ i < f(v)}| ≤ K ?

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
QUESTION: Is there a one-to-one function f: V → {1,2,...,|V|} such that for all i, 1 < i < |V|,

|{{u,v} ∈ E: f(u) ≤ i < f(v)}| ≤ K ?

Reference: [Stockmeyer, 1974b], [Gavril, 1977a]. Transformation from SIMPLE MAX CUT.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
