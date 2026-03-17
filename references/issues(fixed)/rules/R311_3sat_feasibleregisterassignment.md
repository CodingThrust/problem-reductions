---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to FEASIBLE REGISTER ASSIGNMENT"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** FEASIBLE REGISTER ASSIGNMENT
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO2

## GJ Source Entry

> [PO2]  FEASIBLE REGISTER ASSIGNMENT
> INSTANCE:  Directed acyclic graph G = (V,A), positive integer K, and a register assignment f: V→{R1,R2,...,Rk}.
> QUESTION:  Is there a computation for G using the given register assignment, i.e., an ordering v1,v2,...,vn of V and a sequence S0,S1,...,Sn of subsets of V that satisfies all the properties given in REGISTER SUFFICIENCY and that in addition satisfies, for 1≤j≤K and 1≤i≤n, there is at most one vertex u∈Si for which f(u) = Rj?
> Reference:  [Sethi, 1975]. Transformation from 3SAT.
> Comment:  Remains NP-complete even if all vertices of G have out-degree 2 or less.

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