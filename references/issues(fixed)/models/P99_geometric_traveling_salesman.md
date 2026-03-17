---
name: Problem
about: Propose a new problem type
title: "[Model] GeometricTravelingSalesman"
labels: model
assignees: ''
---

## Motivation

GEOMETRIC TRAVELING SALESMAN (P99) from Garey & Johnson, A2 ND23. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND23

**Mathematical definition:**

INSTANCE: Set P ⊆ Z×Z of points in the plane, positive integer B.
QUESTION: Is there a tour of length B or less for the TRAVELING SALESMAN instance with C = P and d((x1,y1),(x2,y2)) equal to the discretized Euclidean distance [((x1−x2)2+(y1−y2)2)½]?

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

INSTANCE: Set P ⊆ Z×Z of points in the plane, positive integer B.
QUESTION: Is there a tour of length B or less for the TRAVELING SALESMAN instance with C = P and d((x1,y1),(x2,y2)) equal to the discretized Euclidean distance [((x1−x2)2+(y1−y2)2)½]?

Reference: [Papadimitriou, 1977] [Garey, Graham, and Johnson, 1976]. Transformation from X3C.
Comment: NP-complete in the strong sense. Remains NP-complete in the strong sense if the distance measure is replaced by the L1 "rectilinear" metric [Garey, Graham, and Johnson, 1976] or the L∞ metric, which is equivalent to L1 under a 45° rotation. Problem remains NP-hard in the strong sense if the (nondiscretized) Euclidean metric is used, but is not known to be in NP [Garey, Graham, and Johnson, 1976].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
