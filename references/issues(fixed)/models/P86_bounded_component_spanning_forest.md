---
name: Problem
about: Propose a new problem type
title: "[Model] BoundedComponentSpanningForest"
labels: model
assignees: ''
---

## Motivation

BOUNDED COMPONENT SPANNING FOREST (P86) from Garey & Johnson, A2 ND10. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND10

**Mathematical definition:**

INSTANCE: Graph G = (V,E), weight w(v) ∈ Z0+ for each v ∈ V, positive integers K ≤ |V| and B.
QUESTION: Can the vertices in V be partitioned into k ≤ K disjoint sets V1,V2,...,Vk such that, for 1 ≤ i ≤ k, the subgraph of G induced by Vi is connected and the sum of the weights of the vertices in Vi does not exceed B?

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

INSTANCE: Graph G = (V,E), weight w(v) ∈ Z0+ for each v ∈ V, positive integers K ≤ |V| and B.
QUESTION: Can the vertices in V be partitioned into k ≤ K disjoint sets V1,V2,...,Vk such that, for 1 ≤ i ≤ k, the subgraph of G induced by Vi is connected and the sum of the weights of the vertices in Vi does not exceed B?

Reference: [Hadlock, 1974]. Transformation from PARTITION INTO PATHS OF LENGTH 2.
Comment: Remains NP-complete even if all weights equal 1 and B is any fixed integer larger than 2 [Garey and Johnson, ——]. Can be solved in polynomial time if G is a tree or if all weights equal 1 and B = 2 [Hadlock, 1974].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
