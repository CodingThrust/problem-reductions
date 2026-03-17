---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Vertex Cover to External Macro Data Compression"
labels: rule
assignees: ''
---

**Source:** Vertex Cover
**Target:** External Macro Data Compression
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.2, p.231

## GJ Source Entry

> [SR22] EXTERNAL MACRO DATA COMPRESSION
> INSTANCE: Alphabet Σ, string s E Σ*, pointer cost h E Z+, and a bound B E Z+.
> QUESTION: Are there strings D (dictionary string) and C (compressed string) in (Σ ∪ {p_i: 1 <= i <= |s|})*, where the symbols p_i are "pointers," such that
>
> |D| + |C| + (h-1)*(number of occurrences of pointers in D and C) <= B
>
> and such that there is a way of identifying pointers with substrings of D so that S can be obtained from C by repeatedly replacing pointers in C by their corresponding substrings in D?
> Reference: [Storer, 1977], [Storer and Szymanski, 1978]. Transformation from VERTEX COVER.
> Comment: Remains NP-complete even if h is any fixed integer 2 or greater. Many variants, including those in which D can contain no pointers and/or no pointers can refer to overlapping strings, are also NP-complete. If the alphabet size is fixed at 3 or greater, and the pointer cost is [h*log|s|], the problem is also NP-complete. For further variants, including the case of "original pointers," see references.

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

- **[Storer, 1977]**: [`Storer1977`] James A. Storer (1977). "{NP}-completeness results concerning data compression". Dept. of Electrical Engineering and Computer Science, Princeton University.
- **[Storer and Szymanski, 1978]**: [`Storer and Szymanski1978`] James A. Storer and Thomas G. Szymanski (1978). "The macro model for data compression (Extended abstract)". In: *Proc. 10th Ann. ACM Symp. on Theory of Computing*, pp. 30–39. Association for Computing Machinery.