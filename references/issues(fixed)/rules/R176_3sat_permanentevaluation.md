---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to PERMANENT EVALUATION"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** PERMANENT EVALUATION
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.3, p.252

## GJ Source Entry

> [AN13] PERMANENT EVALUATION (*)
> INSTANCE: An n×n matrix M of 0's and 1's, and a positive integer K ≤ n!.
> QUESTION: Is the value of the permanent of M equal to K?
> Reference: [Valiant, 1977a]. Transformation from 3SAT.
> Comment: The problem is NP-hard but not known to be in NP, as is the case for the variants in which we ask whether the value of the permanent is "K or less" or "K or more." The problem of computing the value of the permanent of M is #P-complete.

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

- **[Valiant, 1977a]**: [`Valiant1977a`] Leslie G. Valiant (1977). "The complexity of computing the permanent". Computer Science Department, University of Edinburgh.