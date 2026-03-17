---
name: Problem
about: Propose a new problem type
title: "[Model] MaximumLengthBoundedDisjointPaths"
labels: model
assignees: ''
---

## Motivation

MAXIMUM LENGTH-BOUNDED DISJOINT PATHS (P117) from Garey & Johnson, A2 ND41. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND41

**Mathematical definition:**

INSTANCE: Graph G = (V,E), specified vertices s and t, positive integers J,K ≤ |V|.
QUESTION: Does G contain J or more mutually vertex-disjoint paths from s to t, none involving more than K edges?

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

INSTANCE: Graph G = (V,E), specified vertices s and t, positive integers J,K ≤ |V|.
QUESTION: Does G contain J or more mutually vertex-disjoint paths from s to t, none involving more than K edges?
Reference: [Itai, Perl, and Shiloach, 1977]. Transformation from 3SAT.
Comment: Remains NP-complete for all fixed K ≥ 5. Solvable in polynomial time for K ≤ 4. Problem where paths need only be edge-disjoint is NP-complete for all fixed K ≥ 5, polynomially solvable for K ≤ 3, and open for K = 4. The same results hold if G is a directed graph and the paths must be directed paths. The problem of finding the maximum number of disjoint paths from s to t, under no length constraint, is solvable in polynomial time by standard network flow techniques in both the vertex-disjoint and edge-disjoint cases.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
