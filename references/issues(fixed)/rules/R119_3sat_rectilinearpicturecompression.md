---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to Rectilinear Picture Compression"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** Rectilinear Picture Compression
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.2, p.232

## GJ Source Entry

> [SR25] RECTILINEAR PICTURE COMPRESSION
> INSTANCE: An n×n matrix M of 0's and 1's, and a positive integer K.
> QUESTION: Is there a collection of K or fewer rectangles that covers precisely those entries in M that are 1's, i.e., is there a sequence of quadruples (a_i, b_i, c_i, d_i), 1 <= i <= K, where a_i <= b_i, c_i <= d_i, 1 <= i <= K, such that for every pair (i,j), 1 <= i,j <= n, M_{ij} = 1 if and only if there exists a k, 1 <= k <= K, such that a_k <= i <= b_k and c_k <= j <= d_k?
> Reference: [Masek, 1978]. Transformation from 3SAT.

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

- **[Masek, 1978]**: [`Masek1978`] William J. Masek (1978). "Some {NP}-complete set covering problems".