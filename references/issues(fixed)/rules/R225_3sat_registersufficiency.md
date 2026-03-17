---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to REGISTER SUFFICIENCY"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** REGISTER SUFFICIENCY
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, PO1

## GJ Source Entry

> [PO1]  REGISTER SUFFICIENCY
> INSTANCE:  Directed acyclic graph G = (V,A), positive integer K.
> QUESTION:  Is there a computation for G that uses K or fewer registers, i.e., an ordering v1,v2,...,vn of the vertices in V, where n = |V|, and a sequence S0,S1,...,Sn of subsets of V, each satisfying |Si| ≤ K, such that S0 is empty, Sn contains all vertices with in-degree 0 in G, and, for 1 ≤ i ≤ n, vi ∈ Si, Si-{vi} ⊆ Si-1, and Si-1 contains all vertices u for which (vi,u) ∈ A?
> Reference:  [Sethi, 1975]. Transformation from 3SAT.
> Comment:  Remains NP-complete even if all vertices of G have out-degree 2 or less. The variant in which "recomputation" is allowed (i.e., we ask for sequences v1,v2,...,vm and S0,S1,...,Sm, where no a priori bound is placed on m and the vertex sequence can contain repeated vertices, but all other properties stated above must hold) is NP-hard and is not known to be in NP.

## Reduction Algorithm

(TBD)

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Sethi, 1975]**: [`Sethi1975`] R. Sethi (1975). "Complete register allocation problems". *SIAM Journal on Computing* 4, pp. 226–248.