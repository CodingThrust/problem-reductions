---
name: Problem
about: Propose a new problem type
title: "[Model] FeedbackVertexSet"
labels: model
assignees: ''
---

## Motivation

FEEDBACK VERTEX SET (P18) from Garey & Johnson, A1.1 GT7. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT7

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A), positive integer K ≤ |V|.
QUESTION: Is there a subset V' ⊆ V with |V'| ≤ K such that V' contains at least one vertex from every directed cycle in G?

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

INSTANCE: Directed graph G = (V,A), positive integer K ≤ |V|.
QUESTION: Is there a subset V' ⊆ V with |V'| ≤ K such that V' contains at least one vertex from every directed cycle in G?
Reference: [Karp, 1972]. Transformation from VERTEX COVER.
Comment: Remains NP-complete for digraphs having no in- or out-degree exceeding 2, for planar digraphs with no in- or out-degree exceeding 3 [Garey and Johnson, ——], and for edge digraphs [Gavril, 1977a], but can be solved in polynomial time for reducible graphs [Shamir, 1977]. The corresponding problem for undirected graphs is also NP-complete.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
