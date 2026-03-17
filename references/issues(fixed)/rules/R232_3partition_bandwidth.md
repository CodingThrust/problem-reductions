---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3-PARTITION to BANDWIDTH"
labels: rule
assignees: ''
---

**Source:** 3-PARTITION
**Target:** BANDWIDTH
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.3 GT40

## Reduction Algorithm

> INSTANCE: Graph G = (V,E), positive integer K ≤ |V|.
> QUESTION: Is there a linear ordering of V with bandwidth K or less, i.e., a one-to-one function f: V → {1,2,...,|V|} such that, for all {u,v} ∈ E, |f(u)-f(v)| ≤ K?
>
> Reference: [Papadimitriou, 1976a]. Transformation from 3-PARTITION.
>
> Comment: Remains NP-complete for trees with no vertex degree exceeding 3 [Garey, Graham, Johnson, and Knuth, 1978]. This problem corresponds to that of minimizing the "bandwidth" of a symmetric matrix by simultaneous row and column permutations.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Papadimitriou, 1976a]**: [`Papadimitriou1976a`] Christos H. Papadimitriou (1976). "The {NP}-completeness of the bandwidth minimization problem". *Computing* 16, pp. 263–270.
- **[Garey, Graham, Johnson, and Knuth, 1978]**: [`Garey1978a`] M. R. Garey and R. L. Graham and D. S. Johnson and D. E. Knuth (1978). "Complexity results for bandwidth minimization". *SIAM Journal on Applied Mathematics* 34, pp. 477–495.