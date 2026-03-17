---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3-PARTITION to DIRECTED BANDWIDTH"
labels: rule
assignees: ''
---

**Source:** 3-PARTITION
**Target:** DIRECTED BANDWIDTH
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A1.3 GT41

## Reduction Algorithm

> INSTANCE: Directed graph G = (V,A), positive integer K ≤ |V|.
> QUESTION: Is there a one-to-one function f: V → {1,2,...,|V|} such that, for all (u,v) ∈ A, f(u) < f(v) and (f(v)-f(u)) ≤ K?
>
> Reference: [Garey, Graham, Johnson, and Knuth, 1978]. Transformation from 3-PARTITION.
>
> Comment: Remains NP-complete for rooted directed trees with maximum in-degree 1 and maximum out-degree at most 2. This problem corresponds to that of minimizing the "bandwidth" of an upper triangular matrix by simultaneous row and column permutations.

## Size Overhead

| Target metric (code name) | Polynomial (using symbols above) |
|----------------------------|----------------------------------|
| (TBD) | (TBD) |

## Validation Method

(TBD)

## Example

(TBD)


## References

- **[Garey, Graham, Johnson, and Knuth, 1978]**: [`Garey1978a`] M. R. Garey and R. L. Graham and D. S. Johnson and D. E. Knuth (1978). "Complexity results for bandwidth minimization". *SIAM Journal on Applied Mathematics* 34, pp. 477–495.