---
name: Problem
about: Propose a new problem type
title: "[Model] IntersectionGraphForSegmentsOnAGrid"
labels: model
assignees: ''
---

## Motivation

INTERSECTION GRAPH FOR SEGMENTS ON A GRID (P122) from Garey & Johnson, A2 ND46. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND46

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integers M,N.
QUESTION: Is G the intersection graph for a set of line segments on an M×N grid, i.e., is there a one-to-one function f that maps each v ∈ V to a line segment f(v) = [(x,y),(z,w)], where 1 ≤ x ≤ z ≤ M, 1 ≤ y ≤ w ≤ N, and either x = z or y = w, such that {u,v} ∈ E if and only if the line segments f(u) and f(v) intersect?

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

INSTANCE: Graph G = (V,E), positive integers M,N.
QUESTION: Is G the intersection graph for a set of line segments on an M×N grid, i.e., is there a one-to-one function f that maps each v ∈ V to a line segment f(v) = [(x,y),(z,w)], where 1 ≤ x ≤ z ≤ M, 1 ≤ y ≤ w ≤ N, and either x = z or y = w, such that {u,v} ∈ E if and only if the line segments f(u) and f(v) intersect?
Reference: [Gavril, 1977a]. Transformation from 3-PARTITION.
Comment: The analogous problem, which asks if G is the intersection graph for a set of rectangles on an M×N grid, is also NP-complete [Gavril, 1977a].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
