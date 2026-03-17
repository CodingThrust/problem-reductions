---
name: Problem
about: Propose a new problem type
title: "[Model] DegreeConstrainedSpanningTree"
labels: model
assignees: ''
---

## Motivation

DEGREE CONSTRAINED SPANNING TREE (P77) from Garey & Johnson, A2 ND1. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND1

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
QUESTION: Is there a spanning tree for G in which no vertex has degree larger than K?

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

INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
QUESTION: Is there a spanning tree for G in which no vertex has degree larger than K?

Reference: Transformation from HAMILTONIAN PATH.
Comment: Remains NP-complete for any fixed K ≥ 2.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
