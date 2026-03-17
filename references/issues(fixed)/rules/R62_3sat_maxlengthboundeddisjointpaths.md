---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to MAXIMUM LENGTH-BOUNDED DISJOINT PATHS"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** MAXIMUM LENGTH-BOUNDED DISJOINT PATHS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, ND41, p.217

## GJ Source Entry

> [ND41] MAXIMUM LENGTH-BOUNDED DISJOINT PATHS
> INSTANCE: Graph G=(V,E), specified vertices s and t, positive integers J,K≤|V|.
> QUESTION: Does G contain J or more mutually vertex-disjoint paths from s to t, none involving more than K edges?
> Reference: [Itai, Perl, and Shiloach, 1977]. Transformation from 3SAT.
> Comment: Remains NP-complete for all fixed K≥5. Solvable in polynomial time for K≤4. Problem where paths need only be edge-disjoint is NP-complete for all fixed K≥5, polynomially solvable for K≤3, and open for K=4. The same results hold if G is a directed graph and the paths must be directed paths. The problem of finding the maximum number of disjoint paths from s to t, under no length constraint, is solvable in polynomial time by standard network flow techniques in both the vertex-disjoint and edge-disjoint cases.

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

- **[Itai, Perl, and Shiloach, 1977]**: [`Itai1977b`] Alon Itai and Yehoshua Perl and Yossi Shiloach (1977). "The complexity of finding maximum disjoint paths with length constraints". Dept. of Computer Science, Technion.