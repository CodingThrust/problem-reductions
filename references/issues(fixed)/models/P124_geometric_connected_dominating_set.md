---
name: Problem
about: Propose a new problem type
title: "[Model] GeometricConnectedDominatingSet"
labels: model
assignees: ''
---

## Motivation

GEOMETRIC CONNECTED DOMINATING SET (P124) from Garey & Johnson, A2 ND48. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND48

**Mathematical definition:**

INSTANCE: Set P ⊆ Z×Z of points in the plane, positive integers B and K.
QUESTION: Is there a subset P' ⊆ P with |P'| ≤ K such that all points in P − P' are within Euclidean distance B of some point in P', and such that the graph G = (P',E), with an edge between two points in P' if and only if they are within distance B of each other, is connected?

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

INSTANCE: Set P ⊆ Z×Z of points in the plane, positive integers B and K.
QUESTION: Is there a subset P' ⊆ P with |P'| ≤ K such that all points in P − P' are within Euclidean distance B of some point in P', and such that the graph G = (P',E), with an edge between two points in P' if and only if they are within distance B of each other, is connected?
Reference: [Lichtenstein, 1977]. Transformation from PLANAR 3SAT.
Comment: Remains NP-complete if the Euclidean metric is replaced by the L_1 rectilinear metric or the L_∞ metric [Garey and Johnson, ——].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
