---
name: Problem
about: Propose a new problem type
title: "[Model] MultipleCopyFileAllocation"
labels: model
assignees: ''
---

## Motivation

MULTIPLE COPY FILE ALLOCATION (P154) from Garey & Johnson, A4 SR6. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR6

**Mathematical definition:**

INSTANCE: Graph G = (V,E), for each v ∈ V a usage u(v) ∈ Z+ and a storage cost s(v) ∈ Z+, and a positive integer K.
QUESTION: Is there a subset V' ⊆ V such that, if for each v ∈ V we let d(v) denote the number of edges in the shortest path in G from v to a member of V', we have
∑v ∈ V' s(v) + ∑v ∈ V d(v)·u(v) ≤ K ?

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

INSTANCE: Graph G = (V,E), for each v ∈ V a usage u(v) ∈ Z+ and a storage cost s(v) ∈ Z+, and a positive integer K.
QUESTION: Is there a subset V' ⊆ V such that, if for each v ∈ V we let d(v) denote the number of edges in the shortest path in G from v to a member of V', we have
∑v ∈ V' s(v) + ∑v ∈ V d(v)·u(v) ≤ K ?
Reference: [Van Sickle and Chandy, 1977]. Transformation from VERTEX COVER.
Comment: NP-complete in the strong sense, even if all v ∈ V have the same value of u(v) and the same value of s(v).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
