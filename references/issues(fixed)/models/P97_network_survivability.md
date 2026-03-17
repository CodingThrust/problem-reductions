---
name: Problem
about: Propose a new problem type
title: "[Model] NetworkSurvivability"
labels: model
assignees: ''
---

## Motivation

NETWORK SURVIVABILITY (P97) from Garey & Johnson, A2 ND21. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND21

**Mathematical definition:**

INSTANCE: Graph G = (V,E), a rational "failure probability" p(x), 0 ≤ p(x) ≤ 1, for each x ∈ V∪E, a positive rational number q ≤ 1.
QUESTION: Assuming all edge and vertex failures are independent of one another, is the probability q or greater that for all {u,v} ∈ E at least one of u, v, or {u,v} will fail?

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

INSTANCE: Graph G = (V,E), a rational "failure probability" p(x), 0 ≤ p(x) ≤ 1, for each x ∈ V∪E, a positive rational number q ≤ 1.
QUESTION: Assuming all edge and vertex failures are independent of one another, is the probability q or greater that for all {u,v} ∈ E at least one of u, v, or {u,v} will fail?

Reference: [Rosenthal, 1974]. Transformation from VERTEX COVER.
Comment: Not known to be in NP.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
