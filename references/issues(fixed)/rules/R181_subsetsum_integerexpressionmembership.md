---
name: Rule
about: Propose a new reduction rule
title: "[Rule] SUBSET SUM to INTEGER EXPRESSION MEMBERSHIP"
labels: rule
assignees: ''
---

**Source:** SUBSET SUM
**Target:** INTEGER EXPRESSION MEMBERSHIP
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.3, p.253

## GJ Source Entry

> [AN18] INTEGER EXPRESSION MEMBERSHIP
> INSTANCE: Integer expression e over the operations ∪ and +, where if n E Z+, the binary representation of n is an integer expression representing n, and if f and g are integer expressions representing the sets F and G, then f ∪ g is an integer expression representing the set F ∪ G and f + g is an integer expression representing the set {m + n: m E F and n E G}, and a positive integer K.
> QUESTION: Is K in the set represented by e?
> Reference: [Stockmeyer and Meyer, 1973]. Transformation from SUBSET SUM.
> Comment: The related INTEGER EXPRESSION INEQUIVALENCE problem, "given two integer expressions e and f, do they represent different sets?" is NP-hard and in fact complete for Σ_2^p in the polynomial hierarchy ([Stockmeyer and Meyer, 1973], [Stockmeyer, 1976a], see also Section 7.2). If the operator "¬" is allowed, with ¬e representing the set of all positive integers not represented by e, then both the membership and inequivalence problems become PSPACE-complete [Stockmeyer and Meyer, 1973].

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

- **[Stockmeyer and Meyer, 1973]**: [`Stockmeyer and Meyer1973`] Larry J. Stockmeyer and Albert R. Meyer (1973). "Word problems requiring exponential time". In: *Proc. 5th Ann. ACM Symp. on Theory of Computing*, pp. 1–9. Association for Computing Machinery.
- **[Stockmeyer, 1976a]**: [`Stockmeyer1976a`] Larry J. Stockmeyer (1976). "The polynomial-time hierarchy". *Theoretical Computer Science* 3, pp. 1–22.