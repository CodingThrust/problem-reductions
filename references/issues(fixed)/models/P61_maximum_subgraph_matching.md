---
name: Problem
about: Propose a new problem type
title: "[Model] MaximumSubgraphMatching"
labels: model
assignees: ''
---

## Motivation

MAXIMUM SUBGRAPH MATCHING (P61) from Garey & Johnson, A1.4 GT50. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.4 GT50

**Mathematical definition:**

INSTANCE: Directed graphs G = (V_1,A_1), H = (V_2,A_2), positive integer K.
QUESTION: Is there a subset R ⊆ V_1×V_2 with |R| ≥ K such that, for all <u,u'>, <v,v'> ∈ R, (u,v) ∈ A_1 if and only if (u',v') ∈ A_2?

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

INSTANCE: Directed graphs G = (V_1,A_1), H = (V_2,A_2), positive integer K.
QUESTION: Is there a subset R ⊆ V_1×V_2 with |R| ≥ K such that, for all <u,u'>, <v,v'> ∈ R, (u,v) ∈ A_1 if and only if (u',v') ∈ A_2?

Reference: [Garey and Johnson, ——]. Transformation from CLIQUE. Problem is discussed in [Barrow and Burstall, 1976].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
