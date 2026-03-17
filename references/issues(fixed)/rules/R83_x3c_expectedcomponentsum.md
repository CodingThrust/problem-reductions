---
name: Rule
about: Propose a new reduction rule
title: "[Rule] EXACT COVER BY 3-SETS to EXPECTED COMPONENT SUM"
labels: rule
assignees: ''
---

**Source:** EXACT COVER BY 3-SETS
**Target:** EXPECTED COMPONENT SUM
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, SP18, p.224-225

## GJ Source Entry

> [SP18] EXPECTED COMPONENT SUM
> INSTANCE: Collection C of m-dimensional vectors v=(v_1,v_2,…,v_m) with non-negative integer entries, positive integers K and B.
> QUESTION: Is there a partition of C into disjoint sets C_1,C_2,…,C_K such that
> Σ_{i=1}^{K} max_{1≤j≤m}(Σ_{v∈C_i} v_j) ≥ B ?
> Reference: [Garey and Johnson, ——]. Transformation from X3C. The problem is due to [Witsenhausen, 1978] and corresponds to finding a partition that maximizes the expected value of the largest component sum, assuming all sets in the partition are equally likely.
> Comment: NP-complete even if all entries are 0's and 1's. Solvable in polynomial time if K is fixed. The variant in which we ask for a partition with K non-empty sets that yields a sum of B or less is NP-complete even if K is fixed at 3 and all entries are 0's and 1's.

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

- **[Garey and Johnson, ——]**: *(not found in bibliography)*
- **[Witsenhausen, 1978]**: [`Witsenhausen1978`] Hans S. Witsenhausen (1978). "Information aspects of stochastic control".