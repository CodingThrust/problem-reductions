---
name: Problem
about: Propose a new problem type
title: "[Model] RegisterSufficiency"
labels: model
assignees: ''
---

## Motivation

REGISTER SUFFICIENCY (P293) from Garey & Johnson, A11 PO1. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO1

**Mathematical definition:**

INSTANCE: Directed acyclic graph G = (V,A), positive integer K.
QUESTION: Is there a computation for G that uses K or fewer registers, i.e., an ordering v1,v2,...,vn of the vertices in V, where n = |V|, and a sequence S0,S1,...,Sn of subsets of V, each satisfying |Si| ≤ K, such that S0 is empty, Sn contains all vertices with in-degree 0 in G, and, for 1 ≤ i ≤ n, vi ∈ Si, Si−{vi} ⊆ Si−1, and Si−1 contains all vertices u for which (vi,u) ∈ A?

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

INSTANCE: Directed acyclic graph G = (V,A), positive integer K.
QUESTION: Is there a computation for G that uses K or fewer registers, i.e., an ordering v1,v2,...,vn of the vertices in V, where n = |V|, and a sequence S0,S1,...,Sn of subsets of V, each satisfying |Si| ≤ K, such that S0 is empty, Sn contains all vertices with in-degree 0 in G, and, for 1 ≤ i ≤ n, vi ∈ Si, Si−{vi} ⊆ Si−1, and Si−1 contains all vertices u for which (vi,u) ∈ A?
Reference: [Sethi, 1975]. Transformation from 3SAT.
Comment: Remains NP-complete even if all vertices of G have out-degree 2 or less. The variant in which "recomputation" is allowed (i.e., we ask for sequences v1,v2,...,vm and S0,S1,...,Sm, where no a priori bound is placed on m and the vertex sequence can contain repeated vertices, but all other properties stated above must hold) is NP-hard and is not known to be in NP.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
