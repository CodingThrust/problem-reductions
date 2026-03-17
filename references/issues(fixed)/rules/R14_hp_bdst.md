---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN PATH to BOUNDED DEGREE SPANNING TREE"
labels: rule
assignees: ''
---

**Source:** HAMILTONIAN PATH
**Target:** BOUNDED DEGREE SPANNING TREE
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Section 3.2.1, p.64

## Reduction Algorithm

> (4) BOUNDED DEGREE SPANNING TREE
> INSTANCE: A graph G=(V,E) and a positive integer K <= |V|-1.
> QUESTION: Is there a spanning tree for G in which no vertex has degree exceeding K, that is, a subset E' ⊆ E such that |E'|=|V|-1, the graph G'=(V,E') is connected, and no vertex in V is included in more than K edges from E'?
>
> Proof: Restrict to HAMILTONIAN PATH by allowing only instances in which K=2.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)
