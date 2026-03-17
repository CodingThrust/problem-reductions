---
name: Rule
about: Propose a new reduction rule
title: "[Rule] HAMILTONIAN CIRCUIT to QUADRATIC ASSIGNMENT PROBLEM"
labels: rule
assignees: ''
---

**Source:** HAMILTONIAN CIRCUIT
**Target:** QUADRATIC ASSIGNMENT PROBLEM
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND43, p.218

## GJ Source Entry

> [ND43] QUADRATIC ASSIGNMENT PROBLEM
> INSTANCE: Non-negative integer costs c_{ij}, 1≤i,j≤n, and distances d_{kl}, 1≤k,l≤m, bound B∈Z^+.
> QUESTION: Is there a one-to-one function f:{1,2,…,n}→{1,2,…,m} such that
> Σ_{i=1}^{n} Σ_{j=1, j≠i}^{n} c_{ij} d_{f(i)f(j)} ≤ B ?
> Reference: [Sahni and Gonzalez, 1976]. Transformation from HAMILTONIAN CIRCUIT.
> Comment: Special case in which each d_{kl}=k−l and all c_{ji}=c_{ij}∈{0,1} is the NP-complete OPTIMAL LINEAR ARRANGEMENT problem. The general problem is discussed, for example, in [Garfinkel and Nemhauser, 1972].

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

- **[Sahni and Gonzalez, 1976]**: [`Gonzalez1976`] T. Gonzalez and S. Sahni (1976). "Open shop scheduling to minimize finish time". *Journal of the Association for Computing Machinery* 23, pp. 665–679.
- **[Garfinkel and Nemhauser, 1972]**: [`Garfinkel1972`] R. S. Garfinkel and G. L. Nemhauser (1972). "Integer Programming". John Wiley \& Sons, New York.