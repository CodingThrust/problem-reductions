---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to DOMATIC NUMBER"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** DOMATIC NUMBER
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.1 GT3

## GJ Source Entry

> [GT3]  DOMATIC NUMBER
> INSTANCE:  Graph G = (V,E), positive integer K ≤ |V|.
> QUESTION:  Is the domatic number of G at least K, i.e., can V be partitioned into k ≥ K disjoint sets V_1, V_2, ..., V_k such that each V_i is a dominating set for G?
>
> Reference:  [Garey, Johnson, and Tarjan, 1976b]. Transformation from 3SAT. The problem is discussed in [Cockayne and Hedetniemi, 1975].
> Comment:  Remains NP-complete for any fixed K ≥ 3. (The domatic number is always at least 2 unless G contains an isolated vertex.)

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

- **[Garey, Johnson, and Tarjan, 1976b]**: [`Garey1976h`] M. R. Garey and D. S. Johnson and R. E. Tarjan (1976). "The planar {Hamiltonian} circuit problem is {NP}-complete". *SIAM Journal on Computing* 5, pp. 704–714.
- **[Cockayne and Hedetniemi, 1975]**: [`Cockayne1975b`] E. J. Cockayne and S. T. Hedetniemi (1975). "Optimal domination in graphs". *IEEE Transactions on Circuits and Systems* CAS-22, pp. 855–857.