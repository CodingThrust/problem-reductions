---
name: Problem
about: Propose a new problem type
title: "[Model] PartitionIntoHamiltonianSubgraphs"
labels: model
assignees: ''
---

## Motivation

PARTITION INTO HAMILTONIAN SUBGRAPHS (P24) from Garey & Johnson, A1.1 GT13. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT13

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A).
QUESTION: Can the vertices of G be partitioned into disjoint sets V_1, V_2, . . . , V_k, for some k, such that each V_i contains at least three vertices and induces a subgraph of G that contains a Hamiltonian circuit?

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

INSTANCE: Directed graph G = (V,A).
QUESTION: Can the vertices of G be partitioned into disjoint sets V_1, V_2, . . . , V_k, for some k, such that each V_i contains at least three vertices and induces a subgraph of G that contains a Hamiltonian circuit?
Reference: [Valiant, 1977a]. Transformation from 3SAT. (See also [Herrmann, 1973]).
Comment: Solvable in polynomial time by matching techniques if each V_i need only contain at least 2 vertices [Edmonds and Johnson, 1970]. The analogous problem for undirected graphs can be similarly solved, even with the requirement that |V_i| ≥ 3. However, it becomes NP-complete if we require that |V_i| ≥ 6 [Papadimitriou, 1978d] or if the instance includes an upper bound K on k.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
