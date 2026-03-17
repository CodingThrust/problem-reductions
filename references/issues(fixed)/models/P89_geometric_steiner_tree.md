---
name: Problem
about: Propose a new problem type
title: "[Model] GeometricSteinerTree"
labels: model
assignees: ''
---

## Motivation

GEOMETRIC STEINER TREE (P89) from Garey & Johnson, A2 ND13. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND13

**Mathematical definition:**

INSTANCE: Set P ⊆ Z×Z of points in the plane, positive integer K.
QUESTION: Is there a finite set Q ⊆ Z×Z such that there is a spanning tree of total weight K or less for the vertex set P∪Q, where the weight of an edge {(x1,y1),(x2,y2)} is the discretized Euclidean length [((x1−x2)2+(y1−y2)2)½]?

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

INSTANCE: Set P ⊆ Z×Z of points in the plane, positive integer K.
QUESTION: Is there a finite set Q ⊆ Z×Z such that there is a spanning tree of total weight K or less for the vertex set P∪Q, where the weight of an edge {(x1,y1),(x2,y2)} is the discretized Euclidean length [((x1−x2)2+(y1−y2)2)½]?

Reference: [Garey, Graham, and Johnson, 1977]. Transformation from X3C.
Comment: NP-complete in the strong sense. Remains so if the distance measure is replaced by the L1 "rectilinear" metric, |x1−x2|+|y1−y2|, [Garey and Johnson, 1977a] or the L∞ metric, max {|x1−x2|,|y1−y2|}, which is equivalent to L1 under a 45° rotation. Problem remains NP-hard in the strong sense if the (nondiscretized) Euclidean metric ((x1−x2)2+(y1−y2)2)½ is used, but is not known to be in NP [Garey, Graham, and Johnson, 1977]. Some polynomial time algorithms for special cases of the rectilinear case are presented in [Aho, Garey, and Hwang, 1977].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
