---
name: Problem
about: Propose a new problem type
title: "[Model] MinimumKConnectedSubgraph"
labels: model
assignees: ''
---

## Motivation

MINIMUM K-CONNECTED SUBGRAPH (P42) from Garey & Johnson, A1.2 GT31. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.2 GT31

**Mathematical definition:**

INSTANCE: Graph G = (V,E), positive integers K ≤ |V| and B ≤ |E|.
QUESTION: Is there a subset E' ⊆ E with |E'| ≤ B such that G' = (V,E') is K-connected, i.e., cannot be disconnected by removing fewer than K vertices?

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

INSTANCE: Graph G = (V,E), positive integers K ≤ |V| and B ≤ |E|.
QUESTION: Is there a subset E' ⊆ E with |E'| ≤ B such that G' = (V,E') is K-connected, i.e., cannot be disconnected by removing fewer than K vertices?
Reference: [Chung and Graham, 1977]. Transformation from HAMILTONIAN CIRCUIT.
Comment: Corresponding edge-connectivity problem is also NP-complete. Both problems remain NP-complete for any fixed K ≥ 2 and can be solved trivially in polynomial time for K = 1.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
