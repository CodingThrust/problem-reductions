---
name: Problem
about: Propose a new problem type
title: "[Model] NetworkReliability"
labels: model
assignees: ''
---

## Motivation

NETWORK RELIABILITY (P96) from Garey & Johnson, A2 ND20. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND20

**Mathematical definition:**

INSTANCE: Graph G = (V,E), subset V' ⊆ V, a rational "failure probability" p(e), 0 ≤ p(e) ≤ 1, for each e ∈ E, a positive rational number q ≤ 1.
QUESTION: Assuming edge failures are independent of one another, is the probability q or greater that each pair of vertices in V' is joined by at least one path containing no failed edge?

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

INSTANCE: Graph G = (V,E), subset V' ⊆ V, a rational "failure probability" p(e), 0 ≤ p(e) ≤ 1, for each e ∈ E, a positive rational number q ≤ 1.
QUESTION: Assuming edge failures are independent of one another, is the probability q or greater that each pair of vertices in V' is joined by at least one path containing no failed edge?

Reference: [Rosenthal, 1974]. Transformation from STEINER TREE IN GRAPHS.
Comment: Not known to be in NP. Remains NP-hard even if |V'| = 2 [Valiant, 1977b]. The related problem in which we want two disjoint paths between each pair of vertices in V' is NP-hard even if V' = V [Ball, 1977b]. If G is directed and we ask for a directed path between each ordered pair of vertices in V', the one-path problem is NP-hard for both |V'| = 2 [Valiant, 1977b] and V' = V [Ball, 1977a]. Many of the underlying subgraph enumeration problems are #P-complete (see [Valiant, 1977b]).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
