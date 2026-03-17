---
name: Problem
about: Propose a new problem type
title: "[Model] MinimizingDummyActivitiesInPertNetworks"
labels: model
assignees: ''
---

## Motivation

MINIMIZING DUMMY ACTIVITIES IN PERT NETWORKS (P120) from Garey & Johnson, A2 ND44. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND44

**Mathematical definition:**

INSTANCE: Directed acyclic graph G = (V,A) where vertices represent tasks and the arcs represent precedence constraints, and a positive integer K ≤ |V|.
QUESTION: Is there a PERT network corresponding to G with K or fewer dummy activities, i.e., a directed acyclic graph G' = (V',A') where V' = {v_i^−,v_i^+: v ∈ V} and {(v_i^−,v_i^+): v_i ∈ V} ⊆ A', and such that |A'| ≤ |V|+K and there is a path from v_i^+ to v_j^− in G' if and only if there is a path from v_i to v_j in G?

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

INSTANCE: Directed acyclic graph G = (V,A) where vertices represent tasks and the arcs represent precedence constraints, and a positive integer K ≤ |V|.
QUESTION: Is there a PERT network corresponding to G with K or fewer dummy activities, i.e., a directed acyclic graph G' = (V',A') where V' = {v_i^−,v_i^+: v ∈ V} and {(v_i^−,v_i^+): v_i ∈ V} ⊆ A', and such that |A'| ≤ |V|+K and there is a path from v_i^+ to v_j^− in G' if and only if there is a path from v_i to v_j in G?
Reference: [Krishnamoorthy and Deo, 1977b]. Transformation from VERTEX COVER.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
