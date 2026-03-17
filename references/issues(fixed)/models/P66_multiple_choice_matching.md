---
name: Problem
about: Propose a new problem type
title: "[Model] MultipleChoiceMatching"
labels: model
assignees: ''
---

## Motivation

MULTIPLE CHOICE MATCHING (P66) from Garey & Johnson, A1.5 GT55. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.5 GT55

**Mathematical definition:**

INSTANCE: Graph G = (V,E), partition of E into disjoint sets E_1,E_2,...,E_J, positive integer K.
QUESTION: Is there a subset E' ⊆ E with |E'| ≥ K such that no two edges in E' share a common vertex and such that E' contains at most one edge from each E_i, 1 ≤ i ≤ J?

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

INSTANCE: Graph G = (V,E), partition of E into disjoint sets E_1,E_2,...,E_J, positive integer K.
QUESTION: Is there a subset E' ⊆ E with |E'| ≥ K such that no two edges in E' share a common vertex and such that E' contains at most one edge from each E_i, 1 ≤ i ≤ J?

Reference: [Valiant, 1977c], [Itai and Rodeh, 1977a], [Itai, Rodeh, and Tanimota, 1978]. Transformation from 3SAT.
Comment: Remains NP-complete even if G is bipartite, each E_i contains at most 2 edges, and K = |V|/2. If each E_i contains only a single edge, this becomes the ordinary graph matching problem and is solvable in polynomial time.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
