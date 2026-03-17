---
name: Problem
about: Propose a new problem type
title: "[Model] Betweenness"
labels: model
assignees: ''
---

## Motivation

BETWEENNESS (P313) from Garey & Johnson, A12 MS1. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS1

**Mathematical definition:**

INSTANCE: Finite set A, collection C of ordered triples (a,b,c) of distinct elements from A.
QUESTION: Is there a one-to-one function f: A→{1,2,...,|A|} such that for each (a,b,c) ∈ C, we have either f(a) < f(b) < f(c) or f(c) < f(b) < f(a)?

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

INSTANCE: Finite set A, collection C of ordered triples (a,b,c) of distinct elements from A.
QUESTION: Is there a one-to-one function f: A→{1,2,...,|A|} such that for each (a,b,c) ∈ C, we have either f(a) < f(b) < f(c) or f(c) < f(b) < f(a)?
Reference: [Opatrný, 1978]. Transformation from SET SPLITTING.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
