---
name: Problem
about: Propose a new problem type
title: "[Model] GeometricCapacitatedSpanningTree"
labels: model
assignees: ''
---

## Motivation

GEOMETRIC CAPACITATED SPANNING TREE (P82) from Garey & Johnson, A2 ND6. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND6

**Mathematical definition:**

INSTANCE: Set P ⊆ Z×Z of points in the plane, specified point p0 ∈ P, requirement r(p) ∈ Z0+ for each p ∈ P−p0, capacity c ∈ Z+, bound B ∈ Z+.
QUESTION: Is there a spanning tree T = (P,E') for the complete graph G = (P,E) such that ∑e ∈ E' d(e) ≤ B, where d((x1,y1),(x2,y2)) is the discretized Euclidean distance [((x1−x2)2+(y1−y2)2)½], and such that for each e ∈ E', if U(e) is the set of vertices whose paths to p0 pass through e, then ∑u ∈ U(e) r(u) ≤ c?

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

INSTANCE: Set P ⊆ Z×Z of points in the plane, specified point p0 ∈ P, requirement r(p) ∈ Z0+ for each p ∈ P−p0, capacity c ∈ Z+, bound B ∈ Z+.
QUESTION: Is there a spanning tree T = (P,E') for the complete graph G = (P,E) such that ∑e ∈ E' d(e) ≤ B, where d((x1,y1),(x2,y2)) is the discretized Euclidean distance [((x1−x2)2+(y1−y2)2)½], and such that for each e ∈ E', if U(e) is the set of vertices whose paths to p0 pass through e, then ∑u ∈ U(e) r(u) ≤ c?

Reference: [Papadimitriou, 1976c]. Transformation from X3C.
Comment: Remains NP-complete even if all requirements are equal.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
