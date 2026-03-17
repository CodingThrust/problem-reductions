---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Vertex Cover to Internal Macro Data Compression"
labels: rule
assignees: ''
---

**Source:** Vertex Cover
**Target:** Internal Macro Data Compression
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.2, p.231

## GJ Source Entry

> [SR23] INTERNAL MACRO DATA COMPRESSION
> INSTANCE: Alphabet Σ, string s E Σ*, pointer cost h E Z+, and a bound B E Z+.
> QUESTION: Is there a single string C E (Σ ∪ {p_i: 1 <= i <= |s|})* such that
>
> |C| + (h-1)*(number of occurences of pointers in C) <= B
>
> and such that there is a way of identifying pointers with substrings of C so that s can be obtained from C by using C as both compressed string and dictionary string in the manner indicated in the previous problem?
> Reference: [Storer, 1977], [Storer and Szymanski, 1978]. Transformation from VERTEX COVER.
> Comment: Remains NP-complete even if h is any fixed integer 2 or greater. For other NP-complete variants (as in the previous problem), see references.

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