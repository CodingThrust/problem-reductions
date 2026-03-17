---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN CIRCUIT (undirected) to DIRECTED HAMILTONIAN CIRCUIT"
labels: rule
assignees: ''
---

**Source:** HAMILTONIAN CIRCUIT (undirected)
**Target:** DIRECTED HAMILTONIAN CIRCUIT
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Section 3.1.4, p.60

## Reduction Algorithm

> All three Hamiltonian problems mentioned so far also remain NP-complete if we replace the undirected graph G by a directed graph and replace the undirected Hamiltonian circuit or path by a directed Hamiltonian circuit or path. Recall that a directed graph G = (V,A) consists of a vertex set V and a set of ordered pairs of vertices called arcs. A Hamiltonian path in a directed graph G = (V,A) is an ordering of V as <v1,v2, . . . , vn>, where n = |V|, such that (v_i,v_{i+1}) E A for 1 <= i < n. A Hamiltonian circuit has the additional requirement that (v_n,v_1) E A. Each of the three undirected Hamiltonian problems can be transformed to its directed counterpart simply by replacing each edge {u,v} in the given undirected graph by the two arcs (u,v) and (v,u). In essence, the undirected versions are merely special cases of their directed counterparts.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)
