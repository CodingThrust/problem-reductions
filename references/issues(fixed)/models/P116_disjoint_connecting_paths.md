---
name: Problem
about: Propose a new problem type
title: "[Model] DisjointConnectingPaths"
labels: model
assignees: ''
---

## Motivation

DISJOINT CONNECTING PATHS (P116) from Garey & Johnson, A2 ND40. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND40

**Mathematical definition:**

INSTANCE: Graph G = (V,E), collection of disjoint vertex pairs (s_1,t_1),(s_2,t_2),…,(s_k,t_k).
QUESTION: Does G contain k mutually vertex-disjoint paths, one connecting s_i and t_i for each i, 1 ≤ i ≤ k?

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

INSTANCE: Graph G = (V,E), collection of disjoint vertex pairs (s_1,t_1),(s_2,t_2),…,(s_k,t_k).
QUESTION: Does G contain k mutually vertex-disjoint paths, one connecting s_i and t_i for each i, 1 ≤ i ≤ k?
Reference: [Knuth, 1974c], [Karp, 1975a], [Lynch, 1974]. Transformation from 3SAT.
Comment: Remains NP-complete for planar graphs [Lynch, 1974], [Lynch, 1975]. Complexity is open for any fixed k ≥ 2, but can be solved in polynomial time if k = 2 and G is planar or chordal [Perl and Shiloach, 1978]. (A polynomial time algorithm for the general 2 path problem has been announced in [Shiloach, 1978]). The directed version of this problem is also NP-complete in general and solvable in polynomial time when k = 2 and G is planar or acyclic [Perl and Shiloach, 1978].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
