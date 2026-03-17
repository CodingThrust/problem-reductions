---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Partition / 3-Partition to Expected Retrieval Cost"
labels: rule
assignees: ''
---

**Source:** Partition / 3-Partition
**Target:** Expected Retrieval Cost
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.1, p.227

## GJ Source Entry

> [SR4] EXPECTED RETRIEVAL COST
> INSTANCE: Set R of records, rational probability p(r) E [0,1] for each r E R, with sum_{r E R} p(r) = 1, number m of sectors, and a positive integer K.
> QUESTION: Is there a partition of R into disjoint subsets R_1, R_2, ..., R_m such that, if p(R_i) = sum_{r E R_i} p(r) and the "latency cost" d(i,j) is defined to be j-i-1 if 1 <= i < j <= m and to be m-i+j-1 if 1 <= j <= i <= m, then the sum over all ordered pairs i,j, 1 <= i,j <= m, of p(R_i)*p(R_j)*d(i,j) is at most K?
> Reference: [Cody and Coffman, 1976]. Transformation from PARTITION, 3-PARTITION.
> Comment: NP-complete in the strong sense. NP-complete and solvable in pseudo-polynomial time for each fixed m >= 2.

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

- **[Cody and Coffman, 1976]**: [`Cody1976`] R. A. Cody and E. G. Coffman, Jr (1976). "Record allocation for minimizing expected retrieval costs on drum-like storage devices". *Journal of the Association for Computing Machinery* 23, pp. 103–115.