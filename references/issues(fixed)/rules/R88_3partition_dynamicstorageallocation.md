---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3-PARTITION to DYNAMIC STORAGE ALLOCATION"
labels: rule
assignees: ''
---

**Source:** 3-PARTITION
**Target:** DYNAMIC STORAGE ALLOCATION
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, SR2, p.226

## GJ Source Entry

> [SR2] DYNAMIC STORAGE ALLOCATION
> INSTANCE: Set A of items to be stored, each a∈A having a size s(a)∈Z^+, an arrival time r(a)∈Z_0^+, and a departure time d(a)∈Z^+, and a positive integer storage size D.
> QUESTION: Is there a feasible allocation of storage for A, i.e., a function σ: A→{1,2,…,D} such that for every a∈A the allocated storage interval I(a)=[σ(a),σ(a)+s(a)−1] is contained in [1,D] and such that, for all a,a'∈A, if I(a)∩I(a') is nonempty then either d(a)≤r(a') or d(a')≤r(a)?
> Reference: [Stockmeyer, 1976b]. Transformation from 3-PARTITION.
> Comment: NP-complete in the strong sense, even if s(a)∈{1,2} for all a∈A. Solvable in polynomial time if all item sizes are the same, by interval graph coloring algorithms (e.g., see [Gavril, 1972]).

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

- **[Stockmeyer, 1976b]**: [`Stockmeyer1976b`] Larry J. Stockmeyer (1976). "private communication".
- **[Gavril, 1972]**: [`Gavril1972`] F. Gavril (1972). "Algorithms for minimum coloring, maximum clique, minimum covering by cliques, and maximum independent set of a chordal graph". *SIAM Journal on Computing* 1, pp. 180–187.