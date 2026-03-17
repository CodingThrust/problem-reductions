---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Vertex Cover to Minimum Cardinality Key"
labels: rule
assignees: ''
---

**Source:** Vertex Cover
**Target:** Minimum Cardinality Key
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.3, p.232

## GJ Source Entry

> [SR26] MINIMUM CARDINALITY KEY
> INSTANCE: A set A of "attribute names," a collection F of ordered pairs of subsets of A (called "functional dependencies" on A), and a positive integer M.
> QUESTION: Is there a key of cardinality M or less for the relational system <A,F>, i.e., a minimal subset K ⊆ A with |K| <= M such that the ordered pair (K,A) belongs to the "closure" F* of F defined by (1) F ⊆ F*, (2) B ⊆ C ⊆ A implies (C,B) E F*, (3) (B,C),(C,D) E F* implies (B,D) E F*, and (4) (B,C),(B,D) E F* implies (B,C ∪ D) E F*?
> Reference: [Lucchesi and Osborne, 1977], [Lipsky, 1977a]. Transformation from VERTEX COVER. See [Date, 1975] for general background on relational data bases.

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

- **[Lucchesi and Osborne, 1977]**: [`Lucchesi and Osborne1977`] Claudio L. Lucchesi and S. L. Osborne (1977). "Candidate keys for relations". *Journal of Computer and System Sciences*.
- **[Lipsky, 1977a]**: [`Lipsky1977a`] William Lipsky, Jr (1977). "Two {NP}-complete problems related to information retrieval". In: *Fundamentals of Computation Theory*. Springer.
- **[Date, 1975]**: [`Date1975`] C. J. Date (1975). "An Introduction to Database Systems". Addison-Wesley, Reading, MA.