---
name: Problem
about: Propose a new problem type
title: "[Model] CoveringByCompleteBipartiteSubgraphs"
labels: model
assignees: ''
---

## Motivation

COVERING BY COMPLETE BIPARTITE SUBGRAPHS (P29) from Garey & Johnson, A1.1 GT18. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT18

**Mathematical definition:**

INSTANCE: Bipartite graph G = (V,E), positive integer K ≤ |E|.
QUESTION: Are there k ≤ K subsets V_1, V_2, . . . , V_k of V such that each V_i induces a complete bipartite subgraph of G and such that for each edge {u,v} ∈ E there is some V_i that contains both u and v?

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

INSTANCE: Bipartite graph G = (V,E), positive integer K ≤ |E|.
QUESTION: Are there k ≤ K subsets V_1, V_2, . . . , V_k of V such that each V_i induces a complete bipartite subgraph of G and such that for each edge {u,v} ∈ E there is some V_i that contains both u and v?
Reference: [Orlin, 1976]. Transformation from PARTITION INTO CLIQUES.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
