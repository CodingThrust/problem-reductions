---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Hitting Set to Additional Key"
labels: rule
assignees: ''
---

**Source:** Hitting Set
**Target:** Additional Key
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.3, p.232

## GJ Source Entry

> [SR27] ADDITIONAL KEY
> INSTANCE: A set A of attribute names, a collection F of functional dependencies on A, a subset R ⊆ A, and a set K of keys for the relational scheme <R,F>.
> QUESTION: Does R have a key not already contained in K, i.e., is there an R' ⊆ R such that R' ∉ K, (R',R) E F*, and for no R'' ⊆ R' is (R'',R) E F*?
> Reference: [Beeri and Bernstein, 1978]. Transformation from HITTING SET.

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

- **[Beeri and Bernstein, 1978]**: [`Beeri1978`] C. Beeri and P. A. Bernstein (1978). "Computational problems related to the design of normal form relational schemes".