---
name: Problem
about: Propose a new problem type
title: "[Model] ShortestTotalPathLengthSpanningTree"
labels: model
assignees: ''
---

## Motivation

SHORTEST TOTAL PATH LENGTH SPANNING TREE (P79) from Garey & Johnson, A2 ND3. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND3

**Mathematical definition:**

INSTANCE: Graph G = (V,E), integer bound B ∈ Z+.
QUESTION: Is there a spanning tree T for G such that the sum, over all pairs of vertices u,v ∈ V, of the length of the path in T from u to v is no more than K?

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

INSTANCE: Graph G = (V,E), integer bound B ∈ Z+.
QUESTION: Is there a spanning tree T for G such that the sum, over all pairs of vertices u,v ∈ V, of the length of the path in T from u to v is no more than K?

Reference: [Johnson, Lenstra, and Rinnooy Kan, 1978]. Transformation from EXACT COVER BY 3-SETS.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
