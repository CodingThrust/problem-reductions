---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Graph 3-Colorability to Sparse Matrix Compression"
labels: rule
assignees: ''
---

**Source:** Graph 3-Colorability
**Target:** Sparse Matrix Compression
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.2, p.229

## GJ Source Entry

> [SR13] SPARSE MATRIX COMPRESSION
> INSTANCE: An m×n matrix A with entries a_{ij} E {0,1}, 1 <= i <= m, 1 <= j <= n, and a positive integer K <= mn.
> QUESTION: Is there a sequence (b_1, b_2, ..., b_{n+K}) of integers b_i, each satisfying 0 <= b_i <= m, and a function s: {1,2,...,m} → {1,2,...,K} such that, for 1 <= i <= m and 1 <= j <= n, the entry a_{ij} = 1 if and only if b_{s(i)+j-1} = i?
> Reference: [Even, Lichtenstein, and Shiloach, 1977]. Transformation from GRAPH 3-COLORABILITY.
> Comment: Remains NP-complete for fixed K = 3.

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

- **[Even, Lichtenstein, and Shiloach, 1977]**: [`Even1977b`] S. Even and D. I. Lichtenstein and Y. Shiloach (1977). "Remarks on {Zeigler}'s method for matrix compression".