---
name: Problem
about: Propose a new problem type
title: "[Model] MonochromaticTriangle"
labels: model
assignees: ''
---

## Motivation

MONOCHROMATIC TRIANGLE (P17) from Garey & Johnson, A1.1 GT6. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT6

**Mathematical definition:**

INSTANCE: Graph G = (V,E).
QUESTION: Is there a partition of E into two disjoint sets E_1, E_2 such that neither G_1 = (V,E_1) nor G_2 = (V,E_2) contains a triangle?

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

INSTANCE: Graph G = (V,E).
QUESTION: Is there a partition of E into two disjoint sets E_1, E_2 such that neither G_1 = (V,E_1) nor G_2 = (V,E_2) contains a triangle?
Reference: [Burr, 1976]. Transformation from 3SAT.
Comment: Variants in which "triangle" is replaced by any larger fixed complete graph are also NP-complete [Burr, 1976]. Variants in which "triangle" is replaced by "k-star" (a single degree k vertex adjacent to k degree one vertices) is solvable in polynomial time [Burr, Erdös, and Lovasz, 1976].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
