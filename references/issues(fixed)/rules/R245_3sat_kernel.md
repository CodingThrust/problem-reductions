---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to KERNEL"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** KERNEL
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.5 GT57

## Reduction Algorithm

> INSTANCE: Directed graph G = (V,A).
> QUESTION: Does G have a kernel, i.e., a subset V' ⊆ V such that no two vertices in V' are joined by an arc in A and such that for every vertex v ∈ V - V' there is a vertex u ∈ V' for which (u,v) ∈ A?
>
> Reference: [Chvátal, 1973]. Transformation from 3SAT.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Chvátal, 1973]**: [`Chvatal1973`] V. Chv{\'a}tal (1973). "On the computational complexity of finding a kernel". Universit{\'e} de Montr{\'e}al.