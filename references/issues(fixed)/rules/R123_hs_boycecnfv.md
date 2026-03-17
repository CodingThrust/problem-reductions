---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Hitting Set to Boyce-Codd Normal Form Violation"
labels: rule
assignees: ''
---

**Source:** Hitting Set
**Target:** Boyce-Codd Normal Form Violation
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.3, p.233

## GJ Source Entry

> [SR29] BOYCE-CODD NORMAL FORM VIOLATION
> INSTANCE: A set A of attribute names, a collection F of functional dependencies on A, and a subset A' ⊆ A.
> QUESTION: Does A' violate Boyce-Codd normal form for the relational system <A,F>, i.e., is there a subset X ⊆ A' and two attribute names y,z E A' - X such that (X,{y}) E F* and (X,{z}) ∉ F*, where F* is the closure of F?
> Reference: [Bernstein and Beeri, 1976], [Beeri and Bernstein, 1978]. Transformation from HITTING SET.
> Comment: Remains NP-complete even if A' is required to satisfy "third normal form," i.e., if X ⊆ A' is a key for the system <A',F> and if two names y,z E A'-X satisfy (X,{y}) E F* and (X,{z}) ∉ F*, then z is a prime attribute for <A',F>.

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

- **[Bernstein and Beeri, 1976]**: [`Bernstein1976`] P. A. Bernstein and C. Beeri (1976). "An algorithmic approach to normalization of relational database schemas". University of Toronto.
- **[Beeri and Bernstein, 1978]**: [`Beeri1978`] C. Beeri and P. A. Bernstein (1978). "Computational problems related to the design of normal form relational schemes".