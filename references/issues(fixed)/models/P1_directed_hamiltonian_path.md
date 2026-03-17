---
name: Problem
about: Propose a new problem type
title: "[Model] DirectedHamiltonianPath"
labels: model
assignees: ''
---

## Motivation

DIRECTED HAMILTONIAN PATH (P1) from Garey & Johnson, Chapter 3, p.60. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, Chapter 3, p.60

**Mathematical definition:**



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

All three Hamiltonian problems mentioned so far also remain NP-complete if we replace the undirected graph G by a directed graph and replace the undirected Hamiltonian circuit or path by a directed Hamiltonian circuit or path. Recall that a directed graph G = (V, A) consists of a vertex set V and a set of ordered pairs of vertices called arcs. A Hamiltonian path in a directed graph G = (V, A) is an ordering of V as <v_1, v_2, . . . , v_n>, where n = |V|, such that (v_i, v_{i+1}) ∈ A for 1 ≤ i < n. A Hamiltonian circuit has the additional requirement that (v_n, v_1) ∈ A. Each of the three undirected Hamiltonian problems can be transformed to its directed counterpart simply by replacing each edge {u, v} in the given undirected graph by the two arcs (u, v) and (v, u). In essence, the undirected versions are merely special cases of their directed counterparts.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
