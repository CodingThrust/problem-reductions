---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Graph 3-Colorability to 2-Dimensional Consecutive Sets"
labels: rule
assignees: ''
---

**Source:** Graph 3-Colorability
**Target:** 2-Dimensional Consecutive Sets
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.2, p.230

## GJ Source Entry

> [SR19] 2-DIMENSIONAL CONSECUTIVE SETS
> INSTANCE: Finite alphabet Σ, collection C = {Σ_1, Σ_2, ..., Σ_n} of subsets of Σ.
> QUESTION: Is there a partition of Σ into disjoint sets X_1, X_2, ..., X_k such that each X_i has at most one element in common with each Σ_j and such that, for each Σ_j E C, there is an index l(j) such that Σ_j is contained in
>
> X_{l(j)} ∪ X_{l(j)+1} ∪ ... ∪ X_{l(j)+|Σ_j|-1} ?
>
> Reference: [Lipsky, 1977b]. Transformation from GRAPH 3-COLORABILITY.
> Comment: Remains NP-complete if all Σ_j E C have |Σ_j| <= 5, but is solvable in polynomial time if all Σ_j E C have |Σ_j| <= 2.

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

- **[Lipsky, 1977b]**: [`Lipsky1977b`] William Lipsky, Jr (1977). "One more polynomial complete consecutive retrieval problem". *Information Processing Letters* 6, pp. 91–93.