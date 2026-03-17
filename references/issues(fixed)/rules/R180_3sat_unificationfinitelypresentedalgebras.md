---
name: Rule
about: Propose a new reduction rule
title: "[Rule] 3SAT to UNIFICATION FOR FINITELY PRESENTED ALGEBRAS"
labels: rule
assignees: ''
---

**Source:** 3SAT
**Target:** UNIFICATION FOR FINITELY PRESENTED ALGEBRAS
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A7.3, p.253

## GJ Source Entry

> [AN17] UNIFICATION FOR FINITELY PRESENTED ALGEBRAS
> INSTANCE: Finite presentation of an algebra A in terms of a set G of generators, a collection O of operators of various finite dimensions, and a collection Γ of defining relations on well-formed formulas over G and O; two well-formed expressions e and f over G, O, and a variable set V (see reference for details).
> QUESTION: Is there an assignment to each v E V of a unique "term" I(v) over G and O such that, if I(e) and I(f) denote the expressions obtained by replacing all variables in e and f by their corresponding terms, then I(e) and I(f) represent the same element in A?
> Reference: [Kozen, 1977a], [Kozen, 1976]. Transformation from 3SAT. Proof of membership in NP is non-trivial and appears in the second reference.
> Comment: Remains NP-complete if only one of e and f contains variable symbols, but is solvable in polynomial time if neither contains variable symbols. See [Kozen, 1977b] for quantified versions of this problem that are complete for PSPACE and for the various levels of the polynomial hierarchy.

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

- **[Kozen, 1977a]**: [`Kozen1977a`] Dexter Kozen (1977). "Complexity of finitely presented algebras". In: *Proceedings of the 9th Annual ACM Symposium on Theory of Computing*, pp. 164–177. Association for Computing Machinery.
- **[Kozen, 1976]**: [`Kozen1976`] Dexter Kozen (1976). "Complexity of finitely presented algebras". Dept. of Computer Science, Cornell University.
- **[Kozen, 1977b]**: [`Kozen1977b`] Dexter Kozen (1977). "Finitely presented algebras and the polynomial time hierarchy". Dept. of Computer Science, Cornell University.