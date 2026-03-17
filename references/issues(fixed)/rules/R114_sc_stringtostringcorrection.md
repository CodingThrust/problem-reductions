---
name: Rule
about: Propose a new reduction rule
title: "[Rule] Set Covering to String-to-String Correction"
labels: rule
assignees: ''
---

**Source:** Set Covering
**Target:** String-to-String Correction
**Motivation:** (TBD)
**Reference:** Garey & Johnson, *Computers and Intractability*, Appendix A4.2, p.230

## GJ Source Entry

> [SR20] STRING-TO-STRING CORRECTION
> INSTANCE: Finite alphabet Σ, two strings x,y E Σ*, and a positive integer K.
> QUESTION: Is there a way to derive the string y from the string x by a sequence of K or fewer operations of single symbol deletion or adjacent symbol interchange?
> Reference: [Wagner, 1975]. Transformation from SET COVERING.
> Comment: Solvable in polynomial time if the operation set is expanded to include the operations of changing a single character and of inserting a single character, even if interchanges are not allowed (e.g., see [Wagner and Fischer, 1974]), or if the only operation is adjacent symbol interchange [Wagner, 1975]. See reference for related results for cases in which different operations can have different costs.

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

- **[Wagner, 1975]**: [`Wagner1975`] Robert A. Wagner (1975). "On the complexity of the extended string-to-string correction problem". In: *Proc. 7th Ann. ACM Symp. on Theory of Computing*, pp. 218–223. Association for Computing Machinery.
- **[Wagner and Fischer, 1974]**: [`Wagner and Fischer1974`] Robert A. Wagner and Michael J. Fischer (1974). "The string-to-string correction problem". *Journal of the Association for Computing Machinery* 21, pp. 168–173.