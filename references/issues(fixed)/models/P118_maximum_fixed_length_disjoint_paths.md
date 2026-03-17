---
name: Problem
about: Propose a new problem type
title: "[Model] MaximumFixedLengthDisjointPaths"
labels: model
assignees: ''
---

## Motivation

MAXIMUM FIXED-LENGTH DISJOINT PATHS (P118) from Garey & Johnson, A2 ND42. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND42

**Mathematical definition:**

INSTANCE: Graph G = (V,E), specified vertices s and t, positive integers J,K ≤ |V|.
QUESTION: Does G contain J or more mutually vertex-disjoint paths from s to t, each involving exactly K edges?

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
QUESTION: Does G contain J or more mutually vertex-disjoint paths from s to t, each involving exactly K edges?
Reference: [Itai, Perl, and Shiloach, 1977]. Transformation from 3SAT.
Comment: Remains NP-complete for fixed K ≥ 4. Solvable in polynomial time for K ≤ 3. Corresponding problem for edge-disjoint paths is NP-complete for fixed K ≥ 4, polynomially solvable for K ≤ 2, and open for K = 3. The same results hold for directed graphs and directed paths, except that the arc-disjoint version is polynomially solvable for K = 3 and open for K = 4.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
