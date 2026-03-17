---
name: Problem
about: Propose a new problem type
title: "[Model] PartitionIntoIsomorphicSubgraphs"
labels: model
assignees: ''
---

## Motivation

PARTITION INTO ISOMORPHIC SUBGRAPHS (P23) from Garey & Johnson, A1.1 GT12. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT12

**Mathematical definition:**

INSTANCE: Graphs G = (V,E) and H = (V',E') with |V| = q|V'| for some q ∈ Z+.
QUESTION: Can the vertices of G be partitioned into q disjoint sets V_1, V_2, . . . , V_q such that, for 1 ≤ i ≤ q, the subgraph of G induced by V_i is isomorphic to H?

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

INSTANCE: Graphs G = (V,E) and H = (V',E') with |V| = q|V'| for some q ∈ Z+.
QUESTION: Can the vertices of G be partitioned into q disjoint sets V_1, V_2, . . . , V_q such that, for 1 ≤ i ≤ q, the subgraph of G induced by V_i is isomorphic to H?
Reference: [Kirkpatrick and Hell, 1978]. Transformation from 3DM.
Comment: Remains NP-complete for any fixed H that contains at least 3 vertices. The analogous problem in which the subgraph induced by V_i need only have the same number of vertices as H and contain a subgraph isomorphic to H is also NP-complete, for any fixed H that contains a connected component of three or more vertices. Both problems can be solved in polynomial time (by matching) for any H not meeting the stated restrictions.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
