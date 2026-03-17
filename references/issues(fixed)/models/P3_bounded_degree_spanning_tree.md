---
name: Problem
about: Propose a new problem type
title: "[Model] BoundedDegreeSpanningTree"
labels: model
assignees: ''
---

## Motivation

BOUNDED DEGREE SPANNING TREE (P3) from Garey & Johnson, Chapter 3, Section 3.2.1, p.64. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, Chapter 3, Section 3.2.1, p.64

**Mathematical definition:**

INSTANCE: A graph G=(V,E) and a positive integer K ≤ |V|−1.
QUESTION: Is there a spanning tree for G in which no vertex has degree exceeding K, that is, a subset E' ⊆ E such that |E'| = |V|−1, the graph G' = (V,E') is connected, and no vertex in V is included in more than K edges from E'?

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

INSTANCE: A graph G=(V,E) and a positive integer K ≤ |V|−1.
QUESTION: Is there a spanning tree for G in which no vertex has degree exceeding K, that is, a subset E' ⊆ E such that |E'| = |V|−1, the graph G' = (V,E') is connected, and no vertex in V is included in more than K edges from E'?

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
