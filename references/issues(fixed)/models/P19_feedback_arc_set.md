---
name: Problem
about: Propose a new problem type
title: "[Model] FeedbackArcSet"
labels: model
assignees: ''
---

## Motivation

FEEDBACK ARC SET (P19) from Garey & Johnson, A1.1 GT8. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT8

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A), positive integer K ≤ |A|.
QUESTION: Is there a subset A' ⊆ A with |A'| ≤ K such that A' contains at least one arc from every directed cycle in G?

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

INSTANCE: Directed graph G = (V,A), positive integer K ≤ |A|.
QUESTION: Is there a subset A' ⊆ A with |A'| ≤ K such that A' contains at least one arc from every directed cycle in G?
Reference: [Karp, 1972]. Transformation from VERTEX COVER.
Comment: Remains NP-complete for digraphs in which no vertex has total indegree and out-degree more than 3, and for edge digraphs [Gavril, 1977a]. Solvable in polynomial time for planar digraphs [Luchesi, 1976]. The corresponding problem for undirected graphs is trivially solvable in polynomial time.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
