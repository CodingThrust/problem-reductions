---
name: Problem
about: Propose a new problem type
title: "[Model] ConstrainedTriangulation"
labels: model
assignees: ''
---

## Motivation

CONSTRAINED TRIANGULATION (P121) from Garey & Johnson, A2 ND45. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND45

**Mathematical definition:**

INSTANCE: Graph G = (V,E), coordinates x(v), y(v) ∈ Z for each v ∈ V.
QUESTION: Is there a subset E' ⊆ E, such that the set of line segments {[(x(u),y(u)),(x(v),y(v))]: {u,v} ∈ E'} is a triangulation of the set of points {(x(v),y(v)): v ∈ V} in the plane?

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

INSTANCE: Graph G = (V,E), coordinates x(v), y(v) ∈ Z for each v ∈ V.
QUESTION: Is there a subset E' ⊆ E, such that the set of line segments {[(x(u),y(u)),(x(v),y(v))]: {u,v} ∈ E'} is a triangulation of the set of points {(x(v),y(v)): v ∈ V} in the plane?
Reference: [Lloyd, 1977].
Comment: NP-complete in the strong sense.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
