---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3DM to DECODING OF LINEAR CODES"
labels: rule
assignees: ''
---

**Source:** 3DM
**Target:** DECODING OF LINEAR CODES
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS7

## GJ Source Entry

> [MS7]  DECODING OF LINEAR CODES
> INSTANCE:  An n×m matrix A = (aij) of 0's and 1's, a vector ȳ = (y1,y2,...,ym) of 0's and 1's, and a positive integer K.
> QUESTION:  Is there a 0-1 vector x̄ = (x1,x2,...,xn) with no more than K 1's such that, for 1 ≤ j ≤ m, ∑i=1n xi·aij ≡ yj (mod 2)?
> Reference:  [Berlekamp, McEliece, and van Tilborg, 1978]. Transformation from 3DM.
> Comment:  If ȳ is the all zero vector, and hence we are asking for a "codeword" of Hamming weight K or less, the problem is open. The variant in which we ask for an x̄ with exactly K 1's is NP-complete, even for fixed ȳ = (0,0,...,0).

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

- **[Berlekamp, McEliece, and van Tilborg, 1978]**: [`Berlekamp1978`] E. R. Berlekamp and R. J. McEliece and H. C. A. van Tilborg (1978). "On the inherent intractability of certain coding problems". *IEEE Transactions on Information Theory*.