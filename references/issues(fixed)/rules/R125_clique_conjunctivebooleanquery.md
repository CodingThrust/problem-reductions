---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Clique to Conjunctive Boolean Query"
labels: rule
assignees: ''
---

**Source:** Clique
**Target:** Conjunctive Boolean Query
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.3, p.233

## GJ Source Entry

> [SR31] CONJUNCTIVE BOOLEAN QUERY
> INSTANCE: Finite domain set D, a collection R = {R_1, R_2, ..., R_m} of relations, where each R_i consists of a set of d_i-tuples with entries from D, and a conjunctive Boolean query Q over R and D, where such a query Q is of the form
>
> (∃y_1, y_2, ..., y_l)(A_1 ∧ A_2 ∧ ... ∧ A_r)
>
> with each A_i of the form R_j(u_1, u_2, ..., u_{d_j}) where each u E {y_1, y_2, ..., y_l} ∪ D.
> QUESTION: Is Q, when interpreted as a statement about R and D, true?
> Reference: [Chandra and Merlin, 1977]. Transformation from CLIQUE.
> Comment: If we are allowed to replace the conjunctive query Q by an arbitrary first-order sentence involving the predicates in R, then the problem becomes PSPACE-complete, even for D = {0,1}.

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

- **[Chandra and Merlin, 1977]**: [`Chandra1977`] A. K. Chandra and P. M. Merlin (1977). "Optimal implementation of conjunctive queries in relational data bases". In: *Proceedings of the 9th Annual ACM Symposium on Theory of Computing*, pp. 77–90. Association for Computing Machinery.