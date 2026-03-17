---
name: Rule
about: Propose a new reduction rule
title: "[Rule] PARTITION to K-th LARGEST m-TUPLE"
labels: rule
assignees: ''
---

**Source:** PARTITION
**Target:** K-th LARGEST m-TUPLE
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, SP21, p.225

## GJ Source Entry

> [SP21] K^th LARGEST m-TUPLE (*)
> INSTANCE: Sets X_1,X_2,…,X_m⊆Z^+, a size s(x)∈Z^+ for each x∈X_i, 1≤i≤m, and positive integers K and B.
> QUESTION: Are there K or more distinct m-tuples (x_1,x_2,…,x_m) in X_1×X_2×···×X_m for which Σ_{i=1}^{m} s(x_i)≥B?
> Reference: [Johnson and Mizoguchi, 1978]. Transformation from PARTITION.
> Comment: Not known to be in NP. Solvable in polynomial time for fixed m, and in pseudo-polynomial time in general (polynomial in K, Σ|X_i|, and log Σ s(x)). The corresponding enumeration problem is #P-complete.

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

- **[Johnson and Mizoguchi, 1978]**: [`Johnson1978a`] David B. Johnson and Takumi Mizoguchi (1978). "Selecting the $K$th element in $X+Y$ and $X_1+X_2+\cdots+X_m$". *SIAM Journal on Computing* 7, pp. 147–153.